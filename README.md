# libphysis

[![builds.sr.ht status](https://builds.sr.ht/~redstrate/libphysis.svg)](https://builds.sr.ht/~redstrate/libphysis?)

C bindings for [Physis](https://github.com/redstrate/Physis), a library for reading and writing FFXIV data.

## Usage

Simply run `cargo build`, and Cargo will generate the bindings as well pull in Physis to compile into a C-compatible library.

The headers and the libraries should appear under the `target` directory.

### Logger

A logger suitable for connecting to Qt is provided in `logger/`. Simply add_subdirectory to it, and link to the physis-logger target. Then include the header `physis_logger.h` and call the `setup_physis_logging()` function!

## Contributing & Support

The best way you can help is by [monetarily supporting me](https://redstrate.com/fund/) or by submitting patches to
help fix bugs or add functionality. Filing issues is appreciated, but I do this in my free time so please don't expect professional support.

## License

![GPLv3](https://www.gnu.org/graphics/gplv3-127x51.png)

This project is licensed under the [GNU General Public License 3](LICENSE). Some code or assets may be licensed
differently, please refer to the [REUSE](https://reuse.software/spec/) metadata.
