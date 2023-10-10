# libphysis

[![builds.sr.ht status](https://builds.sr.ht/~redstrate/libphysis.svg)](https://builds.sr.ht/~redstrate/libphysis?)

libphysis are C bindings for [physis](https://git.sr.ht/~redstrate/physis), a framework for interacting with FFXIV game data.

## Usage

Simply run `cargo build`, and Cargo will generate the bindings as well pull in physis to compile into a C-compatible library.

The headers and the libraries live under your new `target` directory.

### Logger

A logger suitable for connecting to Qt is provided in `logger/`. Simply add_subdirectory to it, and link to the physis-logger target. Then include the header `physis_logger.h` and call the `setup_physis_logging()` function!

## License

![GPLv3](https://www.gnu.org/graphics/gplv3-127x51.png)

This project is licensed under the [GNU General Public License 3](LICENSE). Some code or assets may be licensed
differently, please refer to the [REUSE](https://reuse.software/spec/) metadata.