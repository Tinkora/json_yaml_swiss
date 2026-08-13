# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.1] - 2026-08-13

### Fixed

- Installed the Rust formatting and lint components required by the release
  verification job.
- Added a workflow contract test that prevents release checks from using
  missing Rust components.

## [0.1.0] - 2026-08-13

The immutable `v0.1.0` candidate tag did not produce a GitHub Release because
its release verification environment lacked required Rust components. Use
`v0.1.1` or later.

### Added

- Strict JSON, YAML, and TOML inspection, advisory detection, normalization,
  and six cross-format conversion directions.
- Versioned WASM reports with stable errors and normalization warnings.
- English-first, Chinese-enabled local browser workbench with file, clipboard,
  and download workflows.
- Resource limits, duplicate-key rejection, unsupported-value checks, and real
  Chromium coverage at four responsive widths.

[Unreleased]: https://github.com/Tinkora/json_yaml_swiss/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/Tinkora/json_yaml_swiss/releases/tag/v0.1.1
[0.1.0]: https://github.com/Tinkora/json_yaml_swiss/tree/v0.1.0
