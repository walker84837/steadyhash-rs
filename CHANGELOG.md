# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic
Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2024-03-17

### Changed

  - Use `thiserror` for internal functions, and use `anyhow` for the main
    function.
  - Update Cargo.toml to get better executables and update version.
  - Update the issue templates to match the project's name.
  - Update dependencies in Cargo.lock
  - Move hashing files to a new module

### Added

  - Add support for Blake2b-256 and Blake2b-512.

### Removed

  - Remove checking for checksum files.

## [0.2.0] - 2024-03-17

### Added

  - Add SHA256, SHA512 and SHA1 checksums.

### Changed

  - Changed the project name from `sha256sum-win-rs` to `steadyhash-rs`.
  - Switch from using the `structopt` crate to `clap`, as it's more updated.

## [0.1.1] - 2023-12-08

### Added

  - Added unit tests in `hashing.rs`.

### Changed

  - Changed from `std::process:ExitCode` to `anyhow::Result` for error handling.
  - Switched from using the `openssl` crate to `sha2` one, as `sha2` is more
    lightweight.

## [0.1.0] - 2023-12-03

### Added

  - Initial release: Basic functionality to compute the SHA256 checksum and
    check it.
