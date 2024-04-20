// SPDX-FileCopyrightText: 2023 Joshua Goins <josh@redstrate.com>
// SPDX-License-Identifier: GPL-3.0-or-later

extern crate cbindgen;

use std::env;

use cbindgen::Language;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.as_str())
        .with_parse_deps(true)
        .with_parse_include(&["physis"])
        .with_language(Language::C)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file("target/public/physis.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir.as_str())
        .with_parse_deps(true)
        .with_parse_include(&["physis"])
        .with_language(Language::Cxx)
        .with_pragma_once(true)
        .generate()
        .expect("Unable to generate C++ bindings")
        .write_to_file("target/public/physis.hpp");
}
