fn main() {
    // apply to all targets
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");

    // only applies to tests
    // println!("cargo:rustc-link-arg-tests=-Tembedded-test.x");
    println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
}

// // use std::env;

// fn main() {
//     // stm32 specific
//     // println!("cargo:rustc-link-arg=-Tlink.x");

//     // add linker script for embedded-test!!
//     println!("cargo::rustc-link-arg-tests=-Tembedded-test.x");
//     println!("cargo::rustc-link-arg-tests=-Tdefmt.x");

//     // Check if the `defmt` feature is enabled, and if so link its linker script
//     // if env::var("CARGO_FEATURE_DEFMT").is_ok() {
//         // println!("cargo:rustc-link-arg=-Tdefmt.x");
//     // }
// }