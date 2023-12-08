# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## \[Unreleased\]

## \[0.1.1\] - 2023-12-08

### Changed

  - Changed from `std::process:ExitCode` to `anyhow::Result` for error handling.
  - Switched from using the `openssl` crate to `sha2` one, as `sha2` is more lightweight.

## \[0.1.0\] - 2023-12-03

### Added

  - Initial release: Basic functionality to compute the SHA256 checksum and check it.
