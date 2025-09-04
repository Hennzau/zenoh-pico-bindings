use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir).join("pico/build/lib");

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=zenohpico");
}
