# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** While below 1.0.0, minor releases may contain breaking changes.

## [Unreleased]

## [0.1.1] - 2026-03-09

### Changed

- Switch from native-tls (OpenSSL) to rustls for TLS, removing the system OpenSSL dependency
- Add portable musl build targets for Linux (x86_64 and aarch64)
- Update all dependencies

### Fixed

- Fix aarch64 cross-compilation ([#1](https://github.com/andrewrabert/aweber-cli/pull/1))

## [0.1.0] - 2026-03-09

Initial release.
