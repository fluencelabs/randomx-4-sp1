use alloy_sol_types::sol;
pub mod randomx_bindings;

use alloy_sol_types::sol_data::Bytes;
use alloy_sol_types::SolStruct;

pub use randomx_bindings::cache::randomx_cache;
pub use randomx_bindings::flags::randomx_flags;

// #[link(name = "randomxflu", kind = "static")]
// extern "C" {
//     pub fn randomx_alloc_cache(flags: randomx_flags) -> *mut randomx_cache;
// }

// sol! {
//     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
//     struct PublicValuesStruct {
//         uint32 n;
//         uint32 a;
//         uint32 b;
//     }
// }

// sol! {
//     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
//     struct PublicValuesStruct {
//          hash;
//     }
// }

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (u32, u32) {
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    (a, b)
}
