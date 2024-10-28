//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolValue;

use unused_lib::randomx_bindings::cache::randomx_cache;
use unused_lib::randomx_bindings::dataset::randomx_dataset;
use unused_lib::randomx_bindings::flags::{randomx_flags, randomx_flags_RANDOMX_FLAG_MICRO};

use unused_lib::randomx_bindings::vm::randomx_vm;
// use unused_lib::{fibonacci, PublicValuesStruct};
use sp1_zkvm::syscalls::sys_panic;

use core::alloc::Layout;
use core::ffi::c_void;
use std::alloc::GlobalAlloc;

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut c_void {
    unsafe {
        let layout = Layout::from_size_align(size, 64).unwrap();
        HEAP.alloc(layout) as *mut c_void
    }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unsafe {
        let layout = Layout::from_size_align(0, 64).unwrap();
        HEAP.dealloc(ptr as *mut u8, layout);
    }
}

// Was unable to remove this undefined symbol from the original called.
// operator delete was called from the pure virtual interface classes:
// randomx_vm and in VmBase dtors.

// operator new[]
// #[no_mangle]
// pub extern "C" fn _Znaj(size: usize) -> *mut c_void {
//     malloc(size)
// }

// operator delete
#[no_mangle]
pub extern "C" fn _ZdlPv(ptr: *mut c_void) {
    free(ptr);
}

// operator delete[]
#[no_mangle]
pub extern "C" fn _ZdaPv(ptr: *mut c_void) {
    free(ptr);
}

#[no_mangle]
pub extern "C" fn abort() {
    unsafe {
        sys_panic("abort".as_ptr(), 5);
    }
}

#[no_mangle]
pub extern "C" fn __assert_func(_file: *const u8, _line: i32, _: *const u8, _: *const u8) {
    unsafe {
        sys_panic("abort".as_ptr(), 5);
    }
}

// To bypass a requirement for static class members in superscalar.cpp.
#[no_mangle]
pub extern "C" fn __cxa_atexit(
    _func: *const c_void,
    _arg: *const c_void,
    _dso_handle: *const c_void,
) -> i32 {
    0
}

#[link(name = "randomxflu", kind = "static")]
extern "C" {
    pub fn randomx_alloc_cache(flags: randomx_flags) -> *mut randomx_cache;
    pub fn randomx_init_cache(cache: *mut randomx_cache, key: *const c_void, key_size: usize);
    pub fn randomx_calculate_hash(
        cache: *mut randomx_vm,
        input: *const c_void,
        input_size: usize,
        output_hash: *mut c_void,
    ) -> *const c_void;
    pub fn randomx_cache_set_micro_cache(
        dst_cache: *mut randomx_cache,
        micro_cache: *const c_void,
        micro_cache_size: usize,
    ) -> *mut randomx_cache;
    pub fn randomx_create_micro_vm(
        flags: randomx_flags,
        cache: *mut randomx_cache,
        dataset: *mut randomx_dataset,
    ) -> *mut randomx_vm;
    pub fn randomx_destroy_vm(vm: *mut c_void);
    pub fn randomx_release_cache(cache: *mut randomx_cache);
}

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let micro_cache = sp1_zkvm::io::read_vec();
    let local_nonce = sp1_zkvm::io::read_vec();
    // let _n = sp1_zkvm::io::read::<u32>();

    // Compute the n'th fibonacci number using a function from the workspace lib crate.
    // let n = my_vec[5] as u32;
    // let (a, b) = fibonacci(n);
    // let randomx_flags = RandomXFlags::recommended(); // WIP
    // let global_nonce = &[0u8; 32];
    // let local_nonce = &[0u8; 32];
    // let _ = compute_randomx_hash(randomx_flags, global_nonce, local_nonce);
    // let c;
    let mut hash = vec![45u8; 32];

    unsafe {
        // let randomx_flags = randomx_flags_RANDOMX_FLAG_DEFAULT;
        let randomx_flags = randomx_flags_RANDOMX_FLAG_MICRO;

        let cache = randomx_alloc_cache(randomx_flags);
        randomx_init_cache(cache, hash.as_ptr() as *const c_void, 32);

        let micro_cache_void_ptr = micro_cache.as_ptr() as *const c_void;
        let cache = randomx_cache_set_micro_cache(cache, micro_cache_void_ptr, micro_cache.len());

        let micro_vm = randomx_create_micro_vm(randomx_flags, cache, std::ptr::null_mut());
        randomx_calculate_hash(
            micro_vm,
            local_nonce.as_ptr() as *const c_void,
            local_nonce.len(),
            hash.as_mut_ptr() as *mut c_void,
        );
    }

    // Encode the public values of the program.
    // let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n: 5, a: 3, b: 4 });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    let bytes = hash.abi_encode();
    sp1_zkvm::io::commit_slice(&bytes);
    // sp1_zkvm::io::commit(&(42 as u32));

    // Need to return micro_cache as well.
    // sp1_zkvm::io::commit_slice(&hash.as_slice());
    // sp1_zkvm::io::commit::<Vec<u8>>(&hash);
}
