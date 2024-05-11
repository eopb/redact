# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.10](https://github.com/eopb/redact/compare/v0.1.9...v0.1.10) - 2024-05-11

### New features
- Implemented `SerializableSecret` for `&Secret<T>`
  This enables `&Secret<T>` to be `Serialized` with `redact::serde::expose_secret`
  and `redact::serde::redact_secret`.

### Other
- Fix issue where dependency versions were not being repeated in dev-dependencies
- Minor documentation improvements

## [0.1.9](https://github.com/eopb/redact/compare/v0.1.8...v0.1.9) - 2024-03-24

### New features
- Support zeroizing secrets with new feature `zeroize`
- Allow `Secret`s to be unsized
- `Secret`s are now `#[repr(transparent)]`
- `serde` feature now supports `no_std`

### Internal
- remove duplicate #[must_use]
- stop repeating versions in dev-dependencies
- use elided lifetimes for `SerializableSecret`
- make `serde` feature explicit
- fixed clippy lints

## [0.1.8](https://github.com/eopb/redact/compare/v0.1.7...v0.1.8) - 2024-01-25

### Added
- `serde::redact_secret` convenience `serialize_with` - serialize without exposing ([#46](https://github.com/eopb/redact/pull/46))

## [0.1.7](https://github.com/eopb/redact/compare/v0.1.6...v0.1.7) - 2023-11-01

### Added
- make associated functions on `Secret` `#[must_use]` ([#42](https://github.com/eopb/redact/pull/42))

## [0.1.6](https://github.com/eopb/redact/compare/v0.1.5...v0.1.6) - 2023-10-28

### Documentation
- enable `generate-link-to-definition` for docs.rs builds ([#38](https://github.com/eopb/redact/pull/38))
- enable `doc_auto_cfg` for docs.rs builds ([#36](https://github.com/eopb/redact/pull/36))

## [0.1.5](https://github.com/eopb/redact/compare/v0.1.4...v0.1.5) - 2023-10-19

### Other
- First release with a CHANGELOG (thanks to [github.com/MarcoIeni/release-plz](https://github.com/MarcoIeni/release-plz))
