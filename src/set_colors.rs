use core::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

#[cfg(feature = "custom-default-colors")]
use atomic_ref::AtomicRef;

use crate::{AnsiColorScheme, DEFAULT_DEFAULT_COLOR_SCHEME};

static SHOULD_COLORIZE: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "custom-default-colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom-default-colors")))]
static DEFAULT_COLOR_SCHEME: AtomicRef<'_, AnsiColorScheme> = AtomicRef::new(None);

/// Enables or disables ANSI colorization.
///
/// Note that this function does not check whether the terminal supports
/// 16-ANSI-color mode or not and doesn't check `NO_COLOR` or `FORCE_COLOR`
/// environment variables. If you need to enable colorization only if supported
/// by the terminal, and you don't need any custom colorization enable
/// conditions, please use [`enable_colors_if_supported`].
///
/// # Examples
///
///
/// ```rust
/// use unwind_context::unwind_context;
///
/// fn func(foo: u32, bar: &str) {
///     let _ctx = unwind_context!(fn(foo, bar));
///     // ...
/// }
/// # /*
/// fn main() {
/// # */
///     unwind_context::set_colors_enabled(true);
/// #   test();
///     // ...
///     func(123, "abc");
///     // ...
/// # /*
/// }
///
/// # */
/// # /*
/// #[test]
/// # */
/// fn test() {
///     unwind_context::set_colors_enabled(true);
///     // ...
///     func(234, "bcd");
///     // ...
/// }
/// ```
#[inline]
pub fn set_colors_enabled(enabled: bool) {
    SHOULD_COLORIZE.store(enabled, AtomicOrdering::Relaxed);
}

#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "renamed to `set_colors_enabled`.")]
pub use set_colors_enabled as set_ansi_colors_enabled;

/// Returns `true` if ANSI colors were enabled before.
///
/// By default colorization is disabled.
///
/// # Examples
///
/// ```rust
/// if unwind_context::are_colors_enabled() {
///     eprintln!("colorization is enabled");
/// } else {
///     eprintln!("colorization is disabled");
/// }
/// ```
#[inline]
pub fn are_colors_enabled() -> bool {
    SHOULD_COLORIZE.load(AtomicOrdering::Relaxed)
}

#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "renamed to `are_colors_enabled`.")]
pub use are_colors_enabled as are_ansi_colors_enabled;

#[cfg(feature = "detect-color-support")]
#[cfg_attr(docsrs, doc(cfg(feature = "detect-color-support")))]
/// Enables ANSI colors if supported by the terminal for stderr stream for all
/// threads.
///
/// It checks for a basic colors support. By default, it enables 16-ANSI-color
/// colorization if the colors have not changed.
///
/// This function uses [`supports-color`] crate to detect color support.
/// [`supports-color`] crate takes the `NO_COLOR` and `FORCE_COLOR` environment
/// variables into account as well.
///
/// [`unwind_context`]: crate::unwind_context
/// [`debug_unwind_context`]: crate::debug_unwind_context
///
/// # Examples
///
/// ```rust
/// use unwind_context::unwind_context;
///
/// fn func(foo: u32, bar: &str) {
///     let _ctx = unwind_context!(fn(foo, bar));
///     // ...
/// }
/// # /*
/// fn main() {
/// # */
///     unwind_context::enable_colors_if_supported();
/// #   test();
///     // ...
///     func(123, "abc");
///     // ...
/// # /*
/// }
///
/// # */
/// # /*
/// #[test]
/// # */
/// fn test() {
///     unwind_context::enable_colors_if_supported();
///     // ...
///     func(234, "bcd");
///     // ...
/// }
/// ```
///
/// [`supports-color`]: https://crates.io/crates/supports-color
#[inline]
pub fn enable_colors_if_supported() {
    use supports_color::Stream;
    if supports_color::on(Stream::Stderr).is_some() {
        set_colors_enabled(true);
    }
}

#[cfg(feature = "detect-color-support")]
#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "renamed to `enable_colors_if_supported`.")]
pub use enable_colors_if_supported as enable_ansi_colors_if_supported;

#[cfg(feature = "custom-default-colors")]
#[cfg_attr(docsrs, doc(cfg(feature = "custom-default-colors")))]
/// Sets default ANSI color scheme for all threads.
///
/// This function uses [`atomic_ref`] crate to modify a static `AtomicRef` with
/// a default ANSI color scheme.
///
/// # Examples
///
/// ```rust
/// unwind_context::set_default_color_scheme(&unwind_context::AnsiColorScheme {
///     default: "\u{1b}[0m",
///     location: "\u{1b}[31m",
///     fn_keyword: "\u{1b}[32m",
///     func_name: "\u{1b}[33m",
///     func_braces: "\u{1b}[34m",
///     value_braces: "\u{1b}[35m",
///     ident: "\u{1b}[36m",
///     item: "\u{1b}[37m",
///     boolean: "\u{1b}[91m",
///     number: "\u{1b}[92m",
///     quoted: "\u{1b}[93m",
///     escaped: "\u{1b}[94m",
/// });
/// ```
///
/// [`atomic_ref`]: https://crates.io/crates/atomic_ref
#[inline]
pub fn set_default_color_scheme(color_scheme: &'static AnsiColorScheme) {
    DEFAULT_COLOR_SCHEME.store(Some(color_scheme), AtomicOrdering::Release);
}

#[cfg(feature = "custom-default-colors")]
#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "renamed to `set_default_color_scheme`.")]
pub use set_default_color_scheme as set_ansi_color_scheme;

/// Returns the currently set default ANSI color scheme.
///
/// # Examples
///
/// ```rust
/// if unwind_context::are_colors_enabled() {
///     eprintln!("colorization is enabled");
/// } else {
///     eprintln!("colorization is disabled");
/// }
///
/// let color_scheme = unwind_context::get_default_color_scheme();
/// eprintln!("color scheme: {:?}", color_scheme);
/// ```
#[inline]
#[must_use]
pub fn get_default_color_scheme() -> &'static AnsiColorScheme {
    get_default_ansi_color_scheme_impl()
}

#[doc(hidden)]
#[deprecated(since = "0.2.0", note = "renamed to `get_default_color_scheme`.")]
pub use get_default_color_scheme as get_ansi_color_scheme;

#[cfg(feature = "custom-default-colors")]
#[inline]
fn get_default_ansi_color_scheme_impl() -> &'static AnsiColorScheme {
    DEFAULT_COLOR_SCHEME
        .load(AtomicOrdering::Acquire)
        .unwrap_or(&DEFAULT_DEFAULT_COLOR_SCHEME)
}

#[cfg(not(feature = "custom-default-colors"))]
#[inline]
fn get_default_ansi_color_scheme_impl() -> &'static AnsiColorScheme {
    &DEFAULT_DEFAULT_COLOR_SCHEME
}

/// Returns current ANSI color scheme if ANSI colors were enabled, `None`
/// otherwise.
///
/// # Examples
///
/// ```rust
/// if let Some(color_scheme) = unwind_context::get_default_color_scheme_if_enabled() {
///     eprintln!(
///         "colorization is enabled with the following color scheme: {:?}",
///         color_scheme
///     );
/// } else {
///     eprintln!("colorization is disabled");
/// }
/// ```
#[inline]
#[must_use]
pub fn get_default_color_scheme_if_enabled() -> Option<&'static AnsiColorScheme> {
    are_colors_enabled().then(get_default_color_scheme)
}

#[doc(hidden)]
#[deprecated(
    since = "0.2.0",
    note = "renamed to `get_default_color_scheme_if_enabled`."
)]
pub use get_default_color_scheme_if_enabled as get_ansi_color_scheme_if_colors_enabled;

#[cfg(all(test, feature = "std"))]
mod tests {
    #[cfg(all(feature = "std", feature = "detect-color-support"))]
    use crate::enable_colors_if_supported;
    use crate::test_common::{SERIAL_TEST, TEST_COLOR_SCHEME};
    use crate::test_util::FixedBufWriter;
    use crate::{
        are_colors_enabled, set_colors_enabled, unwind_context_with_fmt, StdPanicDetector,
    };
    #[cfg(feature = "custom-default-colors")]
    use crate::{set_default_color_scheme, DEFAULT_DEFAULT_COLOR_SCHEME};

    #[test]
    fn test_set_ansi_colors_enabled() {
        let _guard = SERIAL_TEST.lock().unwrap();

        let mut buffer = [0; 128];
        let foo = 123;
        let bar = "BAR";

        assert!(!are_colors_enabled());

        // Colors are disabled by default.
        let mut writer = FixedBufWriter::new(&mut buffer);
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector
        );
        ctx.print();
        drop(ctx);
        assert!(writer
            .into_str()
            .starts_with("foo: 123, bar: \"BAR\"\n    at "));

        // Colors are used if local color scheme if specified.
        let mut writer = FixedBufWriter::new(&mut buffer);
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
            color_scheme = Some(&TEST_COLOR_SCHEME)
        );
        ctx.print();
        drop(ctx);
        assert!(writer
            .into_str()
            .starts_with("foo: {NUM}123{DEF}, bar: {QUOT}\"BAR\"{DEF}\n    at {LOC}"));

        set_colors_enabled(true);
        assert!(are_colors_enabled());

        // The default color scheme is used if colors are enabled globally.
        let mut writer = FixedBufWriter::new(&mut buffer);
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
        );
        ctx.print();
        drop(ctx);
        assert!(writer.into_str().starts_with(
            "foo: \u{1b}[0;96m123\u{1b}[0m, bar: \u{1b}[0;32m\"BAR\"\u{1b}[0m\n    at \u{1b}[94m"
        ));

        // The local color scheme overrides the global one is used if specified.
        let mut writer = FixedBufWriter::new(&mut buffer);
        assert!(are_colors_enabled());
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
            color_scheme = Some(&TEST_COLOR_SCHEME)
        );
        ctx.print();
        drop(ctx);
        assert!(writer
            .into_str()
            .starts_with("foo: {NUM}123{DEF}, bar: {QUOT}\"BAR\"{DEF}\n    at {LOC}"));

        set_colors_enabled(false);
        assert!(!are_colors_enabled());

        // When colors are disabled, it no longer uses any color scheme.
        let mut writer = FixedBufWriter::new(&mut buffer);
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector
        );
        ctx.print();
        drop(ctx);
        assert!(writer
            .into_str()
            .starts_with("foo: 123, bar: \"BAR\"\n    at "));
    }

    #[cfg(all(feature = "std", feature = "detect-color-support"))]
    #[test]
    fn test_enable_ansi_colors_if_supported() {
        let _guard = SERIAL_TEST.lock().unwrap();

        assert!(!are_colors_enabled());

        std::env::remove_var("FORCE_COLOR");
        std::env::remove_var("NO_COLOR");
        std::env::set_var("IGNORE_IS_TERMINAL", "true");
        std::env::set_var("TERM", "dumb");
        enable_colors_if_supported();
        assert!(!are_colors_enabled());

        std::env::set_var("TERM", "xterm-256color");
        std::env::set_var("COLORTERM", "truecolor");
        enable_colors_if_supported();
        assert!(are_colors_enabled());
        set_colors_enabled(false);

        std::env::set_var("NO_COLOR", "true");
        enable_colors_if_supported();
        assert!(!are_colors_enabled());

        std::env::remove_var("NO_COLOR");
        std::env::set_var("FORCE_COLOR", "true");
        enable_colors_if_supported();
        assert!(are_colors_enabled());
        set_colors_enabled(false);

        set_colors_enabled(false);
        assert!(!are_colors_enabled());
    }

    #[cfg(feature = "custom-default-colors")]
    #[test]
    fn test_set_default_ansi_color_scheme() {
        let _guard = SERIAL_TEST.lock().unwrap();

        let mut buffer = [0; 128];
        let foo = 123;
        let bar = "BAR";

        set_colors_enabled(true);
        assert!(are_colors_enabled());

        // The default color scheme is used if colors are enabled globally.
        let mut writer = FixedBufWriter::new(&mut buffer);
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
        );
        ctx.print();
        drop(ctx);
        assert!(writer.into_str().starts_with(concat!(
            "foo: \u{1b}[0;96m123",
            "\u{1b}[0m, bar: \u{1b}[0;32m\"BAR\"",
            "\u{1b}[0m\n    at \u{1b}[94m"
        )));

        set_default_color_scheme(&TEST_COLOR_SCHEME);

        // The default color scheme can be changed.
        let mut writer = FixedBufWriter::new(&mut buffer);
        assert!(are_colors_enabled());
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
        );
        ctx.print();
        drop(ctx);
        assert!(writer
            .into_str()
            .starts_with("foo: {NUM}123{DEF}, bar: {QUOT}\"BAR\"{DEF}\n    at {LOC}"));

        set_default_color_scheme(&DEFAULT_DEFAULT_COLOR_SCHEME);

        // The default color scheme can be changed.
        let mut writer = FixedBufWriter::new(&mut buffer);
        assert!(are_colors_enabled());
        let mut ctx = unwind_context_with_fmt!(
            (foo, bar),
            writer = &mut writer,
            panic_detector = StdPanicDetector,
        );
        ctx.print();
        drop(ctx);
        assert!(writer.into_str().starts_with(concat!(
            "foo: \u{1b}[0;96m123",
            "\u{1b}[0m, bar: \u{1b}[0;32m\"BAR\"",
            "\u{1b}[0m\n    at \u{1b}[94m"
        )));

        set_colors_enabled(false);
        assert!(!are_colors_enabled());
    }
}
