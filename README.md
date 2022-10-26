# libphysis

[![builds.sr.ht status](https://builds.sr.ht/~redstrate/libphysis.svg)](https://builds.sr.ht/~redstrate/libphysis?)

libphysis are C bindings for [physis](https://git.sr.ht/~redstrate/physis), a framework for interacting with FFXIV game data.

## Usage

Simply run `cargo build`, and Cargo will generate the bindings as well pull in physis to compile into a C-compatible library.

The headers and the libraries live under your new `target` directory.