fn main() {
    println!("cargo:rustc-link-search=/git/RandomX/src/");
    println!("cargo:rerun-if-changed=/git/RandomX/src/librandomxflu.a");
}
