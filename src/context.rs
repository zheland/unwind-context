/// Creates [`UnwindContextWithIo`] with a default writer, panic detector, color
/// scheme , and given function or scope context.
///
/// It uses [`std::io::stderr`] writer, [`StdPanicDetector`] panic detector, and
/// a color scheme determined by the [`get_default_color_scheme_if_enabled`]
/// function. If you want to customize a writer, a panic detector, or a color
/// scheme, use [`unwind_context_with_io`] or [`unwind_context_with_fmt`].
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithIo`] will
/// immediately drop. Note that the colorization is disabled by default and can
/// be enabled
#[cfg_attr(
    feature = "detect-color-support",
    doc = "either by the [`set_colors_enabled`] or [`enable_colors_if_supported`] functions."
)]
#[cfg_attr(
    not(feature = "detect-color-support"),
    doc = "by the [`set_colors_enabled`] function."
)]
#[doc = ""]
/// Passed arguments are lazily formatted. The created wrapper takes ownership
/// of the given arguments, so it may be necessary to use value references,
/// clones, or pass the pre-prepared string representation. It also supports the
/// `...` placeholder to show that some values have been omitted.
///
/// There are three forms of this macro:
/// - Create [`UnwindContextFunc`] with an automatically determined function
///   name and the given attributes as function attributes. The arguments do not
///   have to be the real function arguments.
///
/// ```rust
/// use unwind_context::unwind_context;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _ctx = unwind_context!(fn());
///     let _ctx = unwind_context!(fn(a, &b, c));
///     let _ctx = unwind_context!(fn(a, b.clone(), c));
///     let _ctx = unwind_context!(fn(..., c));
///     let _ctx = unwind_context!(fn(a, ...));
///     let _ctx = unwind_context!(fn(a, ..., c));
///     let _ctx = unwind_context!(fn(a, &b, c, "step 1"));
/// }
/// ```
///
/// - Create [`UnwindContextFunc`] with a specific function names and the given
///   attributes as function attributes. Note that only ident-like function
///   names are supported is unquoted. Path names should be enclosed in quotes.
///   The arguments do not have to be the real function arguments.
///
/// ```rust
/// use unwind_context::unwind_context;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _ctx = unwind_context!(fn func());
///     let _ctx = unwind_context!(fn func(a, &b, c));
///     let _ctx = unwind_context!(fn func(a, b.clone(), c));
///     let _ctx = unwind_context!(fn func(..., c));
///     let _ctx = unwind_context!(fn func(a, ...));
///     let _ctx = unwind_context!(fn func(a, ..., c));
///     let _ctx = unwind_context!(fn func(a, &b, c, "step 1"));
///     let _ctx = unwind_context!(fn "func"());
///     let _ctx = unwind_context!(fn "mod1::mod2::func"());
///     let _ctx = unwind_context!(fn "mod1::mod2::func"(a, &b, c));
/// }
/// ```
///
/// - Create [`UnwindContextArgs`] with the given scope attributes.
///
/// ```rust
/// use unwind_context::unwind_context;
///
/// fn func(a: u32) {
///     let b = a.to_string();
///     let c = a > 100;
///     let _ctx = unwind_context!(a, &b, c);
///     let _ctx = unwind_context!(a, b.clone(), c);
///     let _ctx = unwind_context!(..., c);
///     let _ctx = unwind_context!(a, ...);
///     let _ctx = unwind_context!(a, ..., c);
///     let _ctx = unwind_context!(a, &b, c, "step 1");
/// }
/// ```
///
/// [`unwind_context_with_io`]: crate::unwind_context_with_io
/// [`unwind_context_with_fmt`]: crate::unwind_context_with_fmt
/// [`UnwindContextWithIo`]: crate::UnwindContextWithIo
/// [`StdPanicDetector`]: crate::StdPanicDetector
/// [`get_default_color_scheme_if_enabled`]: crate::get_default_color_scheme_if_enabled
/// [`set_colors_enabled`]: crate::set_colors_enabled
#[cfg_attr(
    feature = "detect-color-support",
    doc = "[`enable_colors_if_supported`]: crate::enable_colors_if_supported"
)]
/// [`UnwindContextFunc`]: crate::UnwindContextFunc
/// [`UnwindContextArgs`]: crate::UnwindContextArgs
#[macro_export]
macro_rules! unwind_context {
    ( $( $context:tt )* ) => {
        $crate::unwind_context_with_io!(
            ( $($context)* ),
            writer = ::std::io::stderr(),
            panic_detector = $crate::StdPanicDetector,
            color_scheme = $crate::get_default_color_scheme_if_enabled(),
        )
    };
}

/// Creates [`UnwindContextWithIo`] with a default writer, panic detector, color
/// scheme , and given function or scope context in debug builds only.
///
/// It uses [`std::io::stderr`] writer, [`StdPanicDetector`] panic detector, and
/// a color scheme determined by the [`get_default_color_scheme_if_enabled`]
/// function. If you want to customize a writer, a panic detector, or a color
/// scheme, use [`unwind_context_with_io`] or [`unwind_context_with_fmt`].
///
/// The returned unwind context scope guard value should be kept alive as long
/// as unwind context is needed. If unused, the [`UnwindContextWithIo`] will
/// immediately drop. Note that the colorization is disabled by default and can
/// be enabled
#[cfg_attr(
    feature = "detect-color-support",
    doc = "either by the [`set_colors_enabled`] or [`enable_colors_if_supported`] functions."
)]
#[cfg_attr(
    not(feature = "detect-color-support"),
    doc = "by the [`set_colors_enabled`] function."
)]
#[doc = ""]
/// Passed arguments are lazily formatted. The created wrapper takes ownership
/// of the given arguments, so it may be necessary to use value references,
/// clones, or pass the pre-prepared string representation. It also supports the
/// `...` placeholder to show that some values have been omitted.
///
/// An optimized build will generate `()` unless `-C debug-assertions` is passed
/// to the compiler. This makes this macro no-op with the default release
/// profile.
///
/// There are three forms of this macro:
/// - Create [`UnwindContextFunc`] with an automatically determined function
///   name and the given attributes as function attributes. The arguments do not
///   have to be the real function arguments.
///
/// ```rust
/// use unwind_context::debug_unwind_context;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _ctx = debug_unwind_context!(fn());
///     let _ctx = debug_unwind_context!(fn(a, &b, c));
///     let _ctx = debug_unwind_context!(fn(a, b.clone(), c));
///     let _ctx = debug_unwind_context!(fn(..., c));
///     let _ctx = debug_unwind_context!(fn(a, ...));
///     let _ctx = debug_unwind_context!(fn(a, ..., c));
///     let _ctx = debug_unwind_context!(fn(a, &b, c, "step 1"));
///     // ...
/// }
/// ```
///
/// - Create [`UnwindContextFunc`] with a specific function names and the given
///   attributes as function attributes. Note that only ident-like function
///   names are supported is unquoted. Path names should be enclosed in quotes.
///   The arguments do not have to be the real function arguments.
///
/// ```rust
/// use unwind_context::debug_unwind_context;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _ctx = debug_unwind_context!(fn func());
///     let _ctx = debug_unwind_context!(fn func(a, &b, c));
///     let _ctx = debug_unwind_context!(fn func(a, b.clone(), c));
///     let _ctx = debug_unwind_context!(fn func(..., c));
///     let _ctx = debug_unwind_context!(fn func(a, ...));
///     let _ctx = debug_unwind_context!(fn func(a, ..., c));
///     let _ctx = debug_unwind_context!(fn func(a, &b, c, "step 1"));
///     let _ctx = debug_unwind_context!(fn "func"());
///     let _ctx = debug_unwind_context!(fn "mod1::mod2::func"());
///     let _ctx = debug_unwind_context!(fn "mod1::mod2::func"(a, &b, c));
///     // ...
/// }
/// ```
///
/// - Create [`UnwindContextArgs`] with the given scope attributes.
///
/// ```rust
/// use unwind_context::debug_unwind_context;
///
/// fn func(a: u32) {
///     let b = a.to_string();
///     let c = a > 100;
///     let _ctx = debug_unwind_context!(a, &b, c);
///     let _ctx = debug_unwind_context!(a, b.clone(), c);
///     let _ctx = debug_unwind_context!(..., c);
///     let _ctx = debug_unwind_context!(a, ...);
///     let _ctx = debug_unwind_context!(a, ..., c);
///     let _ctx = debug_unwind_context!(a, &b, c, "step 1");
///     // ...
/// }
/// ```
///
/// [`unwind_context_with_io`]: crate::unwind_context_with_io
/// [`unwind_context_with_fmt`]: crate::unwind_context_with_fmt
/// [`UnwindContextWithIo`]: crate::UnwindContextWithIo
/// [`StdPanicDetector`]: crate::StdPanicDetector
/// [`get_default_color_scheme_if_enabled`]: crate::get_default_color_scheme_if_enabled
/// [`set_colors_enabled`]: crate::set_colors_enabled
#[cfg_attr(
    feature = "detect-color-support",
    doc = "[`enable_colors_if_supported`]: crate::enable_colors_if_supported"
)]
/// [`UnwindContextFunc`]: crate::UnwindContextFunc
/// [`UnwindContextArgs`]: crate::UnwindContextArgs
#[macro_export]
macro_rules! debug_unwind_context {
    ( $( $context:tt )* ) => { $crate::debug_unwind_context_impl!( $($context)* ) };
}

#[doc(hidden)]
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_unwind_context_impl {
    ( $( $context:tt )* ) => { $crate::unwind_context!( $($context)* ) };
}

#[doc(hidden)]
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_unwind_context_impl {
    ($($context:tt)*) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    #[allow(clippy::unwrap_used)]
    #[test]
    fn test_unwind_context_without_unwind() {
        fn func1(foo: usize, bar: &str) -> usize {
            let _ctx = unwind_context!(fn(foo, bar));
            func2(foo.checked_mul(2).unwrap(), &bar[1..])
        }

        fn func2(foo: usize, bar: &str) -> usize {
            let _ctx = unwind_context!(fn(foo, bar));
            func3(foo.checked_mul(3).unwrap(), &bar[1..])
        }

        fn func3(foo: usize, bar: &str) -> usize {
            let _ctx = unwind_context!(fn(foo, bar));
            foo.checked_sub(bar.len()).unwrap()
        }

        let result = func1(1000, "abcdef");
        assert_eq!(result, 5996);

        let result = func1(1000, "ab");
        assert_eq!(result, 6000);

        // Only positive cases checked to avoid capturing `stderr`.
        // Negative cases checked separately with `unwind_context_with_io`.
    }
}
