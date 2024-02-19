use core::fmt::{Debug, Write};
use core::panic::Location;

use crate::{AnsiColorScheme, AnsiColored, DebugAnsiColored, PanicDetector};

/// A structure representing a scoped guard with unwind context with
/// [`std::io::Write`] writer.
///
/// If dropped during unwind it will write a message to a given writer
/// containing given function or scope context. If created with
/// [`unwind_context`] it will write to [`std::io::Stderr`].
///
/// When this structure is dropped (falls out of scope) and the current thread
/// is not unwinding, the unwind context will be forgotten.
///
/// [`unwind_context`]: crate::unwind_context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct UnwindContextWithFmt<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> {
    data: T,
    writer: W,
    panic_detector: P,
    color_scheme: Option<&'static AnsiColorScheme>,
    location: &'static Location<'static>,
}

impl<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> Drop
    for UnwindContextWithFmt<W, T, P>
{
    #[inline]
    fn drop(&mut self) {
        if self.panic_detector.is_panicking() {
            self.print();
        }
    }
}

impl<W: Write, T: Debug + DebugAnsiColored, P: PanicDetector> UnwindContextWithFmt<W, T, P> {
    /// Create a new `UnwindContextWithFmt` with the provided
    /// [`core::fmt::Write`] writer, context scope data, and color scheme.
    #[inline]
    #[must_use = "\
        if unused, the `UnwindContextWithFmt` will immediately drop,
        consider binding the `UnwindContextWithFmt` like `let _ctx = ...`.
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

    /// Print context to a writer specified in the `UnwindContextWithFmt`
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
    }
}

/// Creates [`UnwindContextWithFmt`] with a given [`core::fmt::Write`] writer,
/// panic detector, color scheme, and a given function or scope context.
///
/// If not specified it uses [`get_ansi_color_scheme_if_colors_enabled`] as a
/// default color scheme.
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithFmt`] will
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
/// use unwind_context::unwind_context_with_fmt;
///
/// fn example1(foo: u32, bar: &str, secret: &str, custom_writer: &mut String) {
///     let _ctx = unwind_context_with_fmt!(
///         (fn(foo, bar, ...)),
///         writer = custom_writer,
///         panic_detector = unwind_context::StdPanicDetector,
///         color_scheme = None,
///     );
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::{unwind_context_with_fmt, AnsiColorScheme};
///
/// fn example2<W: core::fmt::Write, P: unwind_context::PanicDetector>(
///     foo: u32,
///     bar: &str,
///     custom_writer: &mut W,
///     custom_panic_detector: P,
///     custom_color_scheme: &'static AnsiColorScheme,
/// ) {
///     let _ctx = unwind_context_with_fmt!(
///         (fn(foo, bar)),
///         writer = custom_writer,
///         panic_detector = custom_panic_detector,
///         color_scheme = Some(custom_color_scheme),
///     );
///     // ...
/// }
/// ```
///
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
/// [`get_ansi_color_scheme_if_colors_enabled`]: crate::get_ansi_color_scheme_if_colors_enabled
#[macro_export]
macro_rules! unwind_context_with_fmt {
    (
        ( $( $context:tt )* )
        , writer = $writer:expr
        , panic_detector = $panic_detector:expr
        $(, color_scheme = $color_scheme:expr )?
        $(,)?
    ) => {
        $crate::UnwindContextWithFmt::new(
            $crate::build_unwind_context_data!( $($context)* ),
            $writer,
            $panic_detector,
            $crate::expr_or_default_expr!(
                $( $color_scheme )?,
                $crate::get_ansi_color_scheme_if_colors_enabled()
            ),
        )
    };
}

/// Creates [`UnwindContextWithFmt`] with a given [`core::fmt::Write`] writer,
/// panic detector, color scheme, and a given function or scope context in debug
/// builds only.
///
/// If not specified it uses [`get_ansi_color_scheme_if_colors_enabled`] as a
/// default color scheme.
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithFmt`] will
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
/// For more information about macro arguments, see [`unwind_context_with_fmt`].
/// For more information about context argument, see
/// [`build_unwind_context_data`].
///
/// # Examples
///
/// ```rust
/// use unwind_context::debug_unwind_context_with_fmt;
///
/// fn example1(foo: u32, bar: &str, secret: &str, custom_writer: &mut String) {
///     let _ctx = debug_unwind_context_with_fmt!(
///         (fn(foo, bar, ...)),
///         writer = custom_writer,
///         panic_detector = unwind_context::StdPanicDetector,
///         color_scheme = None,
///     );
///     // ...
/// }
/// ```
///
/// ```rust
/// use unwind_context::{debug_unwind_context_with_fmt, AnsiColorScheme};
///
/// fn example2<W: core::fmt::Write, P: unwind_context::PanicDetector>(
///     foo: u32,
///     bar: &str,
///     custom_writer: &mut W,
///     custom_panic_detector: P,
///     custom_color_scheme: &'static AnsiColorScheme,
/// ) {
///     let _ctx = debug_unwind_context_with_fmt!(
///         (fn(foo, bar)),
///         writer = custom_writer,
///         panic_detector = custom_panic_detector,
///         color_scheme = Some(custom_color_scheme),
///     );
///     // ...
/// }
/// ```
///
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
/// [`get_ansi_color_scheme_if_colors_enabled`]: crate::get_ansi_color_scheme_if_colors_enabled
#[macro_export]
macro_rules! debug_unwind_context_with_fmt {
    ( $( $tokens:tt )* ) => { $crate::debug_unwind_context_with_fmt_impl!( $($tokens)* ) };
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_unwind_context_with_fmt_impl {
    ( $( $tokens:tt )* ) => { $crate::unwind_context_with_fmt!( $($tokens)* ) };
}

#[doc(hidden)]
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_unwind_context_with_fmt_impl {
    ($($tokens:tt)*) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use core::fmt::Result as FmtResult;
    use core::fmt::Write as FmtWrite;
    use core::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
    #[cfg(feature = "std")]
    use std::borrow::ToOwned;
    #[cfg(feature = "std")]
    use std::string::String;
    #[cfg(feature = "std")]
    use std::sync::mpsc;

    use crate::test_common::{check_location_part, TEST_ANSI_COLOR_SCHEME};
    #[cfg(feature = "std")]
    use crate::test_util::collect_string_from_recv;
    use crate::test_util::{FixedBufWriter, PatternMatcher};
    #[cfg(feature = "std")]
    use crate::StdPanicDetector;
    use crate::{AnsiColorScheme, PanicDetector};

    #[derive(Clone, Debug)]
    pub struct DummyPanicDetector<'a> {
        is_panicking: &'a AtomicBool,
    }

    impl<'a> PanicDetector for DummyPanicDetector<'a> {
        fn is_panicking(&self) -> bool {
            self.is_panicking.load(AtomicOrdering::Relaxed)
        }
    }

    #[cfg(feature = "std")]
    #[derive(Clone, Debug)]
    pub struct ChannelWriter(mpsc::Sender<String>);

    #[cfg(feature = "std")]
    impl FmtWrite for ChannelWriter {
        #[allow(clippy::unwrap_used)]
        fn write_str(&mut self, buf: &str) -> FmtResult {
            self.0.send(buf.to_owned()).unwrap();
            Ok(())
        }
    }

    // This function should be ordered before the `func1`, `func2`, and `func3`
    // functions.
    fn get_min_line() -> u32 {
        line!()
    }

    // Separate writers are used to test function without Rust standard library.
    #[allow(clippy::unwrap_used)]
    fn func1<W: FmtWrite, P: Clone + PanicDetector>(
        foo: usize,
        bar: &str,
        writer1: &mut W,
        writer2: &mut W,
        writer3: &mut W,
        panic_detector: P,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = unwind_context_with_fmt!(
            (fn(foo, bar)),
            writer = writer1,
            panic_detector = panic_detector.clone(),
            color_scheme = color_scheme,
        );
        func2(
            foo.checked_mul(2).unwrap(),
            &bar[1..],
            writer2,
            writer3,
            panic_detector,
            color_scheme,
        )
    }

    // Separate writers are used to test function without Rust standard library.
    #[allow(clippy::unwrap_used)]
    fn func2<W: FmtWrite, P: Clone + PanicDetector>(
        foo: usize,
        bar: &str,
        writer2: &mut W,
        writer3: &mut W,
        panic_detector: P,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = unwind_context_with_fmt!(
            (fn(foo, bar)),
            writer = writer2,
            panic_detector = panic_detector.clone(),
            color_scheme = color_scheme,
        );
        func3(
            foo.checked_mul(3).unwrap(),
            &bar[1..],
            writer3,
            panic_detector,
            color_scheme,
        )
    }

    #[allow(clippy::unwrap_used)]
    fn func3<W: FmtWrite, P: PanicDetector>(
        foo: usize,
        bar: &str,
        writer3: &mut W,
        panic_detector: P,
        color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = unwind_context_with_fmt!(
            (fn(foo, bar)),
            writer = writer3,
            panic_detector = panic_detector,
            color_scheme = color_scheme,
        );
        foo.checked_sub(bar.len()).unwrap()
    }

    #[cfg(feature = "std")]
    #[allow(clippy::unwrap_used)]
    fn func_with_debug_unwind_context<W: FmtWrite, P: PanicDetector>(
        foo: usize,
        bar: &str,
        #[allow(unused_variables)] writer: &mut W,
        #[allow(unused_variables)] panic_detector: P,
        #[allow(unused_variables)] color_scheme: Option<&'static AnsiColorScheme>,
    ) -> usize {
        let _ctx = debug_unwind_context_with_fmt!(
            (fn(foo, bar)),
            writer = writer,
            panic_detector = panic_detector,
            color_scheme = color_scheme,
        );
        foo.checked_sub(bar.len()).unwrap()
    }

    // This function should be ordered after the `func1`, `func2`, and `func3`
    // functions.
    fn get_max_line() -> u32 {
        line!()
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_fmt_without_unwind() {
        let is_panicking = AtomicBool::new(false);
        let dummy_panic_detector = DummyPanicDetector {
            is_panicking: &is_panicking,
        };

        let mut buffer1 = [0; 128];
        let mut buffer2 = [0; 128];
        let mut buffer3 = [0; 128];

        let mut writer1 = FixedBufWriter::new(&mut buffer1);
        let mut writer2 = FixedBufWriter::new(&mut buffer2);
        let mut writer3 = FixedBufWriter::new(&mut buffer3);
        let result = func1(
            1000,
            "abcdef",
            &mut writer1,
            &mut writer2,
            &mut writer3,
            dummy_panic_detector.clone(),
            None,
        );
        assert_eq!(result, 5996);
        assert_eq!(writer1.into_str(), "");
        assert_eq!(writer2.into_str(), "");
        assert_eq!(writer3.into_str(), "");

        let mut writer1 = FixedBufWriter::new(&mut buffer1);
        let mut writer2 = FixedBufWriter::new(&mut buffer2);
        let mut writer3 = FixedBufWriter::new(&mut buffer3);
        let result = func1(
            1000,
            "ab",
            &mut writer1,
            &mut writer2,
            &mut writer3,
            dummy_panic_detector.clone(),
            None,
        );
        assert_eq!(result, 6000);
        assert_eq!(writer1.into_str(), "");
        assert_eq!(writer2.into_str(), "");
        assert_eq!(writer3.into_str(), "");

        // Emulate panicking on the first scope guard drop without real panic.

        is_panicking.store(true, AtomicOrdering::Relaxed);

        let mut writer1 = FixedBufWriter::new(&mut buffer1);
        let mut writer2 = FixedBufWriter::new(&mut buffer2);
        let mut writer3 = FixedBufWriter::new(&mut buffer3);
        let result = func1(
            1000,
            "ab",
            &mut writer1,
            &mut writer2,
            &mut writer3,
            dummy_panic_detector.clone(),
            None,
        );
        assert_eq!(result, 6000);

        let output = &mut writer1.into_str();
        output
            .expect_str("fn func1(foo: 1000, bar: \"ab\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");

        let output = &mut writer2.into_str();
        output
            .expect_str("fn func2(foo: 2000, bar: \"b\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");

        let output = &mut writer3.into_str();
        output
            .expect_str("fn func3(foo: 6000, bar: \"\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");
    }

    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_fmt_without_unwind_with_colored_fmt() {
        let is_panicking = AtomicBool::new(false);
        let dummy_panic_detector = DummyPanicDetector {
            is_panicking: &is_panicking,
        };

        let mut buffer1 = [0; 256];
        let mut buffer2 = [0; 256];
        let mut buffer3 = [0; 256];

        // Emulate panicking on the first scope guard drop without real panic.

        is_panicking.store(true, AtomicOrdering::Relaxed);

        let mut writer1 = FixedBufWriter::new(&mut buffer1);
        let mut writer2 = FixedBufWriter::new(&mut buffer2);
        let mut writer3 = FixedBufWriter::new(&mut buffer3);
        let result = func1(
            1000,
            "ab",
            &mut writer1,
            &mut writer2,
            &mut writer3,
            dummy_panic_detector.clone(),
            Some(&TEST_ANSI_COLOR_SCHEME),
        );
        assert_eq!(result, 6000);

        let output = &mut writer1.into_str();
        output
            .expect_str(
                "{FN}fn {FN_NAME}func1{FN_BRACE}({DEF}foo: {NUM}1000{DEF}, bar: \
                 {QUOT}\"ab\"{DEF}{FN_BRACE}){DEF}\n",
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

        let output = &mut writer2.into_str();
        output
            .expect_str(
                "{FN}fn {FN_NAME}func2{FN_BRACE}({DEF}foo: {NUM}2000{DEF}, bar: \
                 {QUOT}\"b\"{DEF}{FN_BRACE}){DEF}\n",
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

        let output = &mut writer3.into_str();
        output
            .expect_str(
                "{FN}fn {FN_NAME}func3{FN_BRACE}({DEF}foo: {NUM}6000{DEF}, bar: \
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
        assert_eq!(*output, "");
    }

    #[cfg(feature = "std")]
    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_with_fmt_with_unwind() {
        let panic_detector = StdPanicDetector;

        let (sender, recv) = mpsc::channel();
        let mut writer1 = ChannelWriter(sender);
        let mut writer2 = writer1.clone();
        let mut writer3 = writer1.clone();

        let result = std::panic::catch_unwind(move || {
            func1(
                1000,
                "a",
                &mut writer1,
                &mut writer2,
                &mut writer3,
                panic_detector,
                None,
            )
        });
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
        let mut writer1 = ChannelWriter(sender);
        let mut writer2 = writer1.clone();
        let mut writer3 = writer1.clone();

        let result = std::panic::catch_unwind(move || {
            func1(
                1000,
                "",
                &mut writer1,
                &mut writer2,
                &mut writer3,
                panic_detector,
                None,
            )
        });
        assert!(result.is_err());
        let output = collect_string_from_recv(&recv);
        let output = &mut output.as_str();
        output
            .expect_str("fn func1(foo: 1000, bar: \"\")\n")
            .unwrap();
        check_location_part(output, "", "", file!(), get_min_line(), get_max_line());
        assert_eq!(*output, "");

        let (sender, recv) = mpsc::channel();
        let mut writer1 = ChannelWriter(sender);
        let mut writer2 = writer1.clone();
        let mut writer3 = writer1.clone();

        let result = std::panic::catch_unwind(move || {
            func1(
                0,
                "abcdef",
                &mut writer1,
                &mut writer2,
                &mut writer3,
                panic_detector,
                None,
            )
        });
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

    #[cfg(feature = "std")]
    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_debug_unwind_context_with_io_without_unwind() {
        let panic_detector = StdPanicDetector;

        let (sender, recv) = mpsc::channel();
        let mut writer = ChannelWriter(sender);

        let result = std::panic::catch_unwind(move || {
            func_with_debug_unwind_context(4, "abc", &mut writer, panic_detector, None)
        });

        assert_eq!(result.unwrap(), 1);
        let output = collect_string_from_recv(&recv);
        assert_eq!(output, "");
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_debug_unwind_context_with_fmt_with_unwind() {
        let panic_detector = StdPanicDetector;

        let (sender, recv) = mpsc::channel();
        let mut writer = ChannelWriter(sender);

        let result = std::panic::catch_unwind(move || {
            func_with_debug_unwind_context(2, "abc", &mut writer, panic_detector, None)
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
        }
        assert_eq!(*output, "");
    }
}
