use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Define paths
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let c_project_path = manifest_dir.join("../randomx/src");

    println!("c_project_path: {:?}", c_project_path);

    let output = Command::new("make")
        .current_dir(&c_project_path)
        .output()
        .expect("Failed to execute make for compiling C code");

    if !output.status.success() {
        eprintln!("Make failed with status: {}", output.status);
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Make failed to build the C code");
    }

    // Link the compiled library
    println!(
        "cargo:rustc-link-search=native={}",
        c_project_path.display()
    );
    println!("cargo:rustc-link-lib=static=randomxflu");
    println!("cargo:rerun-if-changed={}", c_project_path.display());

    // // Generate bindings using bindgen
    // let bindings = bindgen::Builder::default()
    //     .header(include_dir.join("randomx.h").to_str().unwrap())
    //     .clang_arg(format!("-I{}", include_dir.display()))
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");
}
