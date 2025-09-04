fn main() {
    println!("cargo:rustc-link-search=/Users/enzolevan/Documents/zenoh-pico/build/lib");
    println!("cargo:rustc-link-lib=static=zenohpico");
}
