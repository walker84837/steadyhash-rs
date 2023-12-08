# sha256sum-win

`sha256sum-win` is a port of the sha256sum command, written in Rust.
**Disclaimer**: This project is still under heavy development, and breaking changes are expected.

## Installation

Ensure you have [Rust](https://rustup.rs/) installed in order to compile the project. Clone the repository and run:

``` bash
cargo build --release
```

## Usage: examples

Print SHA256 checksum:

``` console
$ sha256sum-win path/to/file
```

Check SHA256 checksum:

``` console
$ sha256sum-win --check path/to/checksum/file.sha256
```

## Support

For help or suggestions, please open an [issue](https://github.com/walker84837/sha256sum-win-rs/issues).

## Contributing

Contributions are welcome\! If you want to contribute, please:

  - Try to use the least amount of `unsafe` blocks. If that's needed make some safe wrapper function around it.
  - I recommend you to use Windows functions (`winapi` or `std::os::windows`) if performance will be better.
  - Prefer using the standard library over reinventing the wheel.
  - Format code with
    ``` console
    rustfmt --edition 2021
    ```
  - If you would like to make big changes (eg. changing libraries for checksums or removing files), open an issue, explaining what you'd like to change, the main reasons as to why, and what is the difference between using it or not.

## License

This project is dual-licensed under the [Apache 2.0](LICENSE_APACHE.md) or [MIT](LICENSE_MIT.md) licenses.

## Project Status

As of now, development is not very active, and updates occur when time permits. Feel free to contribute if interested.
