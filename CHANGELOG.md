# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Add examples to all public items except some internally-used methods.

## [0.2.0] - 2024-02-22
### Changed
- Rename `set_ansi_colors_enabled` to `set_colors_enabled`.
- Rename `are_ansi_colors_enabled` to `are_colors_enabled`.
- Rename `enable_ansi_colors_if_supported` to
  `enable_colors_if_supported`.
- Rename `set_ansi_color_scheme` to `set_default_color_scheme`.
- Rename `get_ansi_color_scheme` to `get_default_color_scheme`.
- Rename `get_ansi_color_scheme_if_colors_enabled` to
  `get_default_color_scheme_if_enabled`.
- Rename `DEFAULT_ANSI_COLOR_SCHEME` to `DEFAULT_DEFAULT_COLOR_SCHEME`.
- Mark renamed items as deprecated and hide from docs.
- Improve `debug_unwind`-like macros docs.
- Improve `DebugAnsiColored::fmt_colored` docs.
- Fix some missing `#[must_use]` function attributes.

## [0.1.0] - 2024-02-19
### Added
- Context definition macros `{debug_|}unwind_context{_with_fmt|_with_io}`
  and other auxiliary macros
- Unwind context scope guard structures `UnwindContextWithFmt` and
  `UnwindContextWithIo` and other auxiliary structures and traits.
- Basic color and style control functions like `set_colors_enabled`,
  `are_colors_enabled` and `get_default_color_scheme`.
- Feature-gated advanced color and style control functions like
  `enable_colors_if_supported` and `set_default_color_scheme`.
- API documentation with examples.
- Tests and doc-tests.
- GitHub CI.

[Unreleased]: https://github.com/zheland/unwind-context/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/zheland/unwind-context/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/zheland/unwind-context/compare/v0.0.0...v0.1.0
