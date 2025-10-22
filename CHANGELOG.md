# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com)
and this project adheres to [Semantic Versioning](http://semver.org).

## [Unreleased] - ReleaseDate

## [0.1.0] - 2025-10-22
- First release of `ReXOR`, a cross-platform Rust library with CLI app implementing XOR encryption/decryption
- Add `encode` and `decode` core functions for XOR encryption/decryption
- Add CLI with interactive menu, secure password input (input is hidden), and file dialogs
- Implement efficient file I/O with 8 KB chunked processing
- Add cross-platform support for Windows, Linux, and macOS
- Add CI/CD pipeline for building and publishing binaries via GitHub Actions
- Embed Windows icon using `rcedit`
- Publish crate to [crates.io](https://crates.io/crates/rexor) with [documentation](https://docs.rs/rexor)

[Unreleased]: https://github.com/ree-verse/ReXOR/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ree-verse/ReXOR/releases/tag/v0.1.0
