# steadyhash: reliable file integrity checker

SteadyHash provides a straightforward way to generate and verify SHA-1, SHA-256,
and SHA-512 checksums.

## Usage

To use this utility, follow these steps:

1.  Ensure you have Rust and Cargo installed. If not, you can install them from
    Rust's [official website](https://www.rust-lang.org/tools/install).
2.  Clone this repository to your local machine.
3.  Navigate to the project directory.
4.  Run `cargo run --release` to run and build the utility.

### Command-line arguments

### Generating checksums

To generate a checksum for a file, use the following command:

Usage: `steadyhash [OPTIONS] --type <CHECKSUM> [FILEs]...`

Arguments: `[FILEs]... : the files to process`

Options:

  - `-t, --type`: the type of SHA checksum. Possible valeus are:
      - 1 (SHA-1)
      - 256 (SHA-256)
      - 512 (SHA-512)
  - `-c, --check`: read checksums from the FILEs and check them
  - `--tag`: create a BSD-style checksum
  - `--binary`: read in binary mode
  - `-s, --stdin`: read data from stdin
  - `-h, --help`: print help
  - `-V, --version`: print version

### Verifying checksums

To verify a checksum against a file, use the following command:

``` console
$ steadyhash --check --type [CHECKSUM_TYPE] [FILE_PATH]
```

## Roadmap

  - \[X\] Support for multiple platforms (as of now only Unix-like are
    supported)
  - \[ \] Support for checking BSD-style checksums
  - \[ \] Configuration (maybe)

## Support

If you encounter any issues or have questions about this utility, feel free to
[open an issue](https://github.com/walker84837/steadyhash-rs/issues).

## License

This project is licensed under the [GPL-3.0](LICENSE.md).

Contributions are welcome. Feel free to fork this repository and submit pull
requests with improvements or bug fixes. If you're unsure about something, open
an issue to discuss it further. Thank you to all contributors who can help make
this project better\!
