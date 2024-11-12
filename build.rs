fn main() {
    // This script will execute at compile time, useful for custom builds
    println!("cargo:rerun-if-changed=build.rs");
}

