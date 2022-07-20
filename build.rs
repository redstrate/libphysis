extern crate cbindgen;

use std::env;
use cbindgen::DocumentationStyle::C99;
use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.as_str())
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("target/public/bindings.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir.as_str())
        .with_language(Language::Cxx)
        .generate()
        .expect("Unable to generate C++ bindings")
        .write_to_file("target/public/bindings.hpp");
}