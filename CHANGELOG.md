# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.5](https://github.com/eopb/redact/compare/v0.1.4...v0.1.5) - 2023-10-19

### Added
- `fake` support

### Other
- Add release-plz action ([#34](https://github.com/eopb/redact/pull/34))
- Update version used in README ([#33](https://github.com/eopb/redact/pull/33))
- Release version `0.1.3` :rocket: ([#32](https://github.com/eopb/redact/pull/32))
- Add CI step to test minimum versions ([#31](https://github.com/eopb/redact/pull/31))
- Add CI check for msrv ([#30](https://github.com/eopb/redact/pull/30))
- Fix missing `>` in docs
- Add note on `SerializableSecret` about making it public ([#29](https://github.com/eopb/redact/pull/29))
- Adjust wording in secrecy comparison ([#28](https://github.com/eopb/redact/pull/28))
- Add keywords ([#27](https://github.com/eopb/redact/pull/27))
- Release version `0.1.2`
- Enable `serde::expose_secret` to deserialize `Option<Secret<T>>` ([#24](https://github.com/eopb/redact/pull/24))
- Add more steps to CI and use `cargo hack` ([#26](https://github.com/eopb/redact/pull/26))
- Split project into modules
- Release version `0.1.1` :rocket:
- pretag
- Release version `0.1.0` :rocket:
- bump patch version
- inline
- from and try_from methods
- docs
- operators and transforms
- don't expose error when using FromStr
- better trait order
- bump patch version
- FromStr> FromStr for Secret<T>
- Merge pull request [#15](https://github.com/eopb/redact/pull/15) from ldbrierley/main
- Implemented Hash, Ord and PartialOrd to Secret
- more See module level documentation
- move api docs link to top
- bump patch version
- convert expose_secret into a simple method rather than a trait method
- bump patch version
- enable no_std
- bump patch version
- Fix typos
- add struct example
- bump patch version
- Merge branch 'main' into docs
- bump patch version
- remove requirement on serde-derive
- Merge pull request [#4](https://github.com/eopb/redact/pull/4) from eopb/fix-cargo-toml-metadata
- bump patch version
- Simplify
- fill in cargo.toml metadata
- add tests
- Inline
- Write initial implementation
- Add GitHub actions workflow
- Initial commit
