use core::fmt::Debug;
use core::panic::Location;
use std::io::Write;

use crate::{AnsiColorScheme, AnsiColored, DebugAnsiColored, PanicDetector};

/// A structure representing a scoped guard with unwind context with
/// [`core::fmt::Write`] writer.
///
/// If dropped during unwind it will write a message to a given writer
/// containing given function or scope context.
///
/// When this structure is dropped (falls out of scope) and the current thread
/// is not unwinding, the unwind context will be forgotten.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnwindContextWithIo<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> {
    data: T,
    writer: W,
    panic_detector: P,
    color_scheme: Option<&'static AnsiColorScheme>,
    location: &'static Location<'static>,
}

impl<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> Drop
    for UnwindContextWithIo<W, T, P>
{
    #[inline]
    fn drop(&mut self) {
        if self.panic_detector.is_panicking() {
            self.print();
        }
    }
}

impl<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> UnwindContextWithIo<W, T, P> {
    /// Create a new `UnwindContextWithFmt` with the provided
    /// [`core::fmt::Write`] writer, context scope data, and color scheme.
    #[inline]
    #[must_use = "\
        if unused, the `UnwindContextWithIo` will immediately drop,
        consider binding the `UnwindContextWithIo` like `let _ctx = ...`.
    "]
    #[track_caller]
    pub fn new(
        data: T,
        writer: W,
        panic_detector: P,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> Self {
        Self {
            data,
            writer,
            panic_detector,
            color_scheme,
            location: Location::caller(),
        }
    }

    /// Print context to a writer specified in the `UnwindContextWithIo`
    /// constructor.
    ///
    /// This method is called when a panic detected.
    #[cold]
    #[inline(never)]
    pub fn print(&mut self) {
        if let Some(color_scheme) = self.color_scheme {
            let _ = writeln!(
                self.writer,
                "{:?}\n    at {}{}:{}:{}{}",
                AnsiColored::new(&self.data, color_scheme),
                color_scheme.location,
                self.location.file(),
                self.location.line(),
                self.location.column(),
                color_scheme.default,
            );
        } else {
            let _ = writeln!(
                self.writer,
                "{:?}\n    at {}:{}:{}",
                self.data,
                self.location.file(),
                self.location.line(),
                self.location.column(),
            );
        }
        let _ = self.writer.flush();
    }
}

/// Creates [`UnwindContextWithIo`] with a given [`std::io::Write`] writer,
/// panic detector, color scheme, and a given function or scope context.
///
/// If not specified it uses [`std::io::stderr`] as a default writer,
/// [`StdPanicDetector`] as a default panic detector and
/// [`get_ansi_color_scheme_if_colors_enabled`] as a default color scheme. When
/// using default values for all optional parameters, consider the use of
/// [`unwind_context`] macro instead. See
/// [equivalent macros](#equivalent-macros) section below.
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithIo`] will
/// immediately drop.
///
/// Passed context arguments are lazily formatted. The created wrapper takes
/// ownership of the given arguments, so it may be necessary to use value
/// references, clones, or pass the pre-prepared string representation. It also
/// supports the `...` placeholder to show that some values have been omitted.
///
/// For more information about context argument, see
/// [`build_unwind_context_data`].
///
/// # Examples
///
/// ```rust
/// use unwind_context::unwind_context_with_io;
///
/// fn example1(foo: u32, bar: &str, secret: &str) {
///     let _ctx = unwind_context_with_io!((fn(foo, bar, ...)), color_scheme = None);
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::unwind_context_with_io;
///
/// fn example2(foo: u32, bar: &str, secret: &str) {
///     let _ctx = unwind_context_with_io!((fn(foo, bar, ...)), writer = ::std::io::stdout());
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::{unwind_context_with_io, AnsiColorScheme};
///
/// fn example3<W: std::io::Write, P: unwind_context::PanicDetector>(
///     foo: u32,
///     bar: &str,
///     custom_writer: &mut W,
///     custom_panic_detector: P,
///     custom_color_scheme: &'static AnsiColorScheme,
/// ) {
///     let _ctx = unwind_context_with_io!(
///         (fn(foo, bar)),
///         writer = custom_writer,
///         panic_detector = custom_panic_detector,
///         color_scheme = Some(custom_color_scheme),
///     );
///     // ...
/// }
/// ```
///
/// # Equivalent macros
/// ```rust
/// use unwind_context::{unwind_context, unwind_context_with_io};
///
/// fn func(foo: u32, bar: &str) {
///     unwind_context!(fn(foo, bar));
///     unwind_context_with_io!((fn(foo, bar)));
///     unwind_context_with_io!(
///         (fn(foo, bar)),
///         writer = ::std::io::stderr(),
///         panic_detector = unwind_context::StdPanicDetector,
///         color_scheme = unwind_context::get_ansi_color_scheme_if_colors_enabled(),
///     );
/// }
/// ```
///
/// [`unwind_context`]: crate::unwind_context
/// [`StdPanicDetector`]: crate::StdPanicDetector
/// [`get_ansi_color_scheme_if_colors_enabled`]: crate::get_ansi_color_scheme_if_colors_enabled
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
#[macro_export]
macro_rules! unwind_context_with_io {
    (
        ( $( $context:tt )* )
        $(, writer = $writer:expr )?
        $(, panic_detector = $panic_detector:expr )?
        $(, color_scheme = $color_scheme:expr )?
        $(,)?
    ) => {
        $crate::UnwindContextWithIo::new(
            $crate::build_unwind_context_data!( $($context)* ),
            $crate::expr_or_default_expr!(
                $( $writer )?,
                ::std::io::stderr()
            ),
            $crate::expr_or_default_expr!(
                $( $panic_detector )?,
                $crate::StdPanicDetector
            ),
            $crate::expr_or_default_expr!(
                $( $color_scheme )?,
                $crate::get_ansi_color_scheme_if_colors_enabled()
            ),
        )
    };
}

/// Creates [`UnwindContextWithIo`] with a given [`std::io::Write`] writer,
/// panic detector, color scheme, and a given function or scope context in debug
/// builds only.
///
/// If not specified it uses [`std::io::stderr`] as a default writer,
/// [`StdPanicDetector`] as a default panic detector and
/// [`get_ansi_color_scheme_if_colors_enabled`] as a default color scheme. When
/// using default values for all optional parameters, consider the use of
/// [`debug_unwind_context`] macro instead. See
/// [equivalent macros](#equivalent-macros) section below.
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithIo`] will
/// immediately drop.
///
/// Passed context arguments are lazily formatted. The created wrapper takes
/// ownership of the given arguments, so it may be necessary to use value
/// references, clones, or pass the pre-prepared string representation. It also
/// supports the `...` placeholder to show that some values have been omitted.
///
/// An optimized build will generate `()` unless `-C debug-assertions` is passed
/// to the compiler. This makes this macro no-op with the default release
/// profile.
///
/// For more information about macro arguments, see [`unwind_context_with_io`].
/// For more information about context argument, see
/// [`build_unwind_context_data`].
///
/// # Examples
///
/// ```rust
/// use unwind_context::debug_unwind_context_with_io;
///
/// fn example1(foo: u32, bar: &str, secret: &str) {
///     let _ctx = debug_unwind_context_with_io!((fn(foo, bar, ...)), color_scheme = None);
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::debug_unwind_context_with_io;
///
/// fn example2(foo: u32, bar: &str, secret: &str) {
///     let _ctx = debug_unwind_context_with_io!((fn(foo, bar, ...)), writer = ::std::io::stdout());
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::{debug_unwind_context_with_io, AnsiColorScheme};
///
/// fn example3<W: std::io::Write, P: unwind_context::PanicDetector>(
///     foo: u32,
///     bar: &str,
///     custom_writer: &mut W,
///     custom_panic_detector: P,
///     custom_color_scheme: &'static AnsiColorScheme,
/// ) {
///     let _ctx = debug_unwind_context_with_io!(
///         (fn(foo, bar)),
///         writer = custom_writer,
///         panic_detector = custom_panic_detector,
///         color_scheme = Some(custom_color_scheme),
///     );
///     // ...
/// }
/// ```
///
/// # Equivalent macros
/// ```rust
/// use unwind_context::{debug_unwind_context, debug_unwind_context_with_io};
///
/// fn func(foo: u32, bar: &str) {
///     debug_unwind_context!(fn(foo, bar));
///     debug_unwind_context_with_io!((fn(foo, bar)));
///     debug_unwind_context_with_io!(
///         (fn(foo, bar)),
///         writer = ::std::io::stderr(),
///         panic_detector = unwind_context::StdPanicDetector,
///         color_scheme = unwind_context::get_ansi_color_scheme_if_colors_enabled(),
///     );
/// }
/// ```
///
/// [`debug_unwind_context`]: crate::debug_unwind_context
/// [`StdPanicDetector`]: crate::StdPanicDetector
/// [`get_ansi_color_scheme_if_colors_enabled`]: crate::get_ansi_color_scheme_if_colors_enabled
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
#[macro_export]
macro_rules! debug_unwind_context_with_io {
    ( $( $tokens:tt )* ) => { $crate::debug_unwind_context_with_io_impl!( $($tokens)* ) };
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_unwind_context_with_io_impl {
    ( $( $tokens:tt )* ) => { $crate::unwind_context_with_io!( $($tokens)* ) };
}

#[doc(hidden)]
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_unwind_context_with_io_impl {
    ($($tokens:tt)*) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::ToOwned;
    use std::io::{Result as IoResult, Write as IoWrite};
    use std::string::String;
    use std::sync::mpsc;

    use crate::test_common::{check_location_part, TEST_ANSI_COLOR_SCHEME};
    use crate::test_util::{collect_string_from_recv, PatternMatcher};
    use crate::AnsiColorScheme;

    #[derive(Clone)]
    pub struct Writer(mpsc::Sender<String>);

    impl IoWrite for Writer {
        #[allow(clippy::unwrap_used)]
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            self.0
                .send(String::from_utf8(buf.to_owned()).unwrap())
                .unwrap();
            Ok(buf.len())
        }

        fn flush(&mut self) -> IoResult<()> {
            Ok(())
        }
    }

    fn get_min_line() -> u32 {
        line!()
    }

    #[allow(clippy::unwrap_used)]
    fn func1<W: Clone + IoWrite>(
        foo: usize,
        bar: &str,
        writer: &mut W,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = unwind_context_with_io!(
            (fn(foo, bar)),
            writer = writer.clone(),
            color_scheme = color_scheme
        );
        func2(foo.checked_mul(2).unwrap(), &bar[1..], writer, color_scheme)
    }

    #[allow(clippy::unwrap_used)]
    fn func2<W: Clone + IoWrite>(
        foo: usize,
        bar: &str,
        writer: &mut W,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = unwind_context_with_io!(
            (fn(foo, bar)),
            writer = writer.clone(),
            color_scheme = color_scheme
        );
        func3(foo.checked_mul(3).unwrap(), &bar[1..], writer, color_scheme)
    }

    #[allow(clippy::unwrap_used)]
    fn func3<W: IoWrite>(
        foo: usize,
        bar: &str,
        writer: &mut W,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx =
            unwind_context_with_io!((fn(foo, bar)), writer = writer, color_scheme = color_scheme);
        foo.checked_sub(bar.len()).unwrap()
    }

    #[allow(clippy::unwrap_used)]
    fn func_with_debug_unwind_context<W: IoWrite>(
        foo: usize,
        bar: &str,
        #[allow(unused_variables)] writer: &mut W,
        #[allow(unused_variables)] color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = debug_unwind_context_with_io!(
            (fn(foo, bar)),
            writer = writer,
            color_scheme = color_scheme
        );
        foo.checked_sub(bar.len()).unwrap()
    }

    fn get_max_line() -> u32 {
        line!()
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_io_without_unwind() {
        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = func1(1000, "abcdef", &mut writer, None);
        assert_eq!(result, 5996);
        assert_eq!(collect_string_from_recv(&recv), "");

        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = func1(1000, "ab", &mut writer, None);
        assert_eq!(result, 6000);
        assert_eq!(collect_string_from_recv(&recv), "");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_io_with_unwind() {
        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || func1(1000, "a", &mut writer, None));
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();
        output
            .expect_str("fn func2(foo: 2000, bar: \"\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        output
            .expect_str("fn func1(foo: 1000, bar: \"a\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");

        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || func1(1000, "", &mut writer, None));
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();
        output
            .expect_str("fn func1(foo: 1000, bar: \"\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");

        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || func1(0, "abcdef", &mut writer, None));
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();
        output
            .expect_str("fn func3(foo: 0, bar: \"cdef\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        output
            .expect_str("fn func2(foo: 0, bar: \"bcdef\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        output
            .expect_str("fn func1(foo: 0, bar: \"abcdef\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_io_with_unwind_with_colored_fmt() {
        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || {
            func1(1000, "a", &mut writer, Some(&TEST_ANSI_COLOR_SCHEME))
        });
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();
        output
            .expect_str(
                "{FN}fn {FN_NAME}func2{FN_BRACE}({DEF}foo: {NUM}2000{DEF}, bar: \
                 {QUOT}\"\"{DEF}{FN_BRACE}){DEF}\n",
            )
            .unwrap();
        check_location_part(
            output,
            "{LOC}",
            "{DEF}",
            file!(),
            get_min_line(),
            get_max_line(),
        );
        output
            .expect_str(
                "{FN}fn {FN_NAME}func1{FN_BRACE}({DEF}foo: {NUM}1000{DEF}, bar: \
                 {QUOT}\"a\"{DEF}{FN_BRACE}){DEF}\n",
            )
            .unwrap();
        check_location_part(
            output,
            "{LOC}",
            "{DEF}",
            file!(),
            get_min_line(),
            get_max_line(),
        );
        assert_eq!(*output, "");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_debug_unwind_context_with_io_without_unwind() {
        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || {
            func_with_debug_unwind_context(4, "abc", &mut writer, None)
        });

        assert_eq!(result.unwrap(), 1);
        let output = collect_string_from_recv(&recv);
        assert_eq!(output, "");
    }

    #[test]
    fn test_debug_unwind_context_with_io_with_unwind() {
        let (sender, recv) = mpsc::channel();
        let mut writer = Writer(sender);
        let result = std::panic::catch_unwind(move || {
            func_with_debug_unwind_context(2, "abc", &mut writer, None)
        });
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();

        #[cfg(debug_assertions)]
        {
            output
                .expect_str("fn func_with_debug_unwind_context(foo: 2, bar: \"abc\")\n")
                .unwrap();
            check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
            assert_eq!(*output, "");
        }
        assert_eq!(*output, "");
    }
}
