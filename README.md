# steadyhash: reliable file integrity checker

SteadyHash provides a straightforward way to generate and verify SHA-1, SHA-256,
SHA-512, Blake2b-512 and Blake2b-256 checksums.

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

  - `-t, --type`: the type of SHA checksum. Possible values are:
    - `sha`
    - `blake`
    - `b2`
    - `blake2`
  - `-l, --length`: the bit length of the checksum:
    - The valid values for `sha` are:
        - `160`
        - `256`
        - `512`
    - The valid values for `blake` are:
        - `256`
        - `512`
  - `-c, --check`: read checksums from the FILEs and check them (unimplemented)
  - `--tag`: create a BSD-style checksum
  - `--binary`: read in binary mode (does nothing)
  - `-s, --stdin`: read data from stdin
  - `-h, --help`: print help
  - `-V, --version`: print version

#### Examples

  - Generate a SHA256 BSD-style checksum:
    ```console
    $ steadyhash -l 256 -t sha foo.bar
    ```

  - Generate a Blake2b-256 checksum:
    ```console
    $ steadyhash -l 256 -t b2 foo.bar
    ```

  - Generate a SHA1 BSD-style checksum:
    ```console
    $ steadyhash -l 160 -t sha --tag foo.bar
    ````

## Roadmap

  - [X] Support for multiple platforms
  - [ ] Support for checking checksums
  - [ ] Configuration (maybe)

## Support

If you encounter any issues or have questions about this utility, feel free to
[open an issue](https://github.com/walker84837/steadyhash/issues).

## License

This project is licensed under the [GPL-3.0](LICENSE.md).

Contributions are welcome. Feel free to fork this repository and submit pull
requests with improvements or bug fixes. If you're unsure about something, open
an issue to discuss it further. Thank you to all contributors who can help make
this project better\!
