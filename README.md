# sha256sum-win

## Description

`sha256sum-win` is a Windows port of the sha256sum command, providing the ability to print or check SHA256 (256-bit) checksums. Please note that this project is still under heavy development, and API-breaking changes are expected. As of now, development is not very active, and updates occur when time permits.

## Features

- Windows port of the sha256sum command
- Print or check SHA256 checksums
- Dual-licensed under Apache 2.0 or MIT license

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Clone the repository and run:

```bash
cargo build --release
```

## Usage

### Print SHA256 checksum

```bash
sha256sum-win path/to/file
```

### Check SHA256 checksum

```bash
sha256sum-win --check path/to/checksum/file
```

## Requirements

- Rust (for building)
- OpenSSL

## Support

For help or suggestions, please open an [issue](https://github.com/walker84837/sha256sum-win/issues).

## Roadmap

No specific roadmap available at the moment.

## Contributing

Contributions are welcome! If you want to contribute, please check the [Contributing Guidelines](CONTRIBUTING.md).

## Authors and Acknowledgment

- [walker84837](https://github.com/walker84837) - Sole developer

## License

This project is dual-licensed under the [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT) licenses.

## Project Status

Development is not very active. Feel free to fork or contribute if interested.
