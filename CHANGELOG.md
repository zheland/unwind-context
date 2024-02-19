# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-02-19
### Added
- Context definition macros `{debug_|}unwind_context{_with_fmt|_with_io}` and other auxiliary macros
- Unwind context scope guard structures `UnwindContextWithFmt` and `UnwindContextWithIo` and other auxiliary structures and traits.
- Basic color and style control functions like `set_ansi_colors_enabled`, `are_ansi_colors_enabled` and `get_ansi_color_scheme`.
- Feature-gated advanced color and style control functions like `enable_ansi_colors_if_supported` and `set_ansi_color_scheme`.
- API documentation with examples.
- Tests and doc-tests.

[Unreleased]: https://github.com/zheland/custom-print/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/zheland/custom-print/compare/v0.0.0...v0.1.0
