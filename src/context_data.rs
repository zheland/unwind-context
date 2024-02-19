/// Creates either [`UnwindContextFunc`] or [`UnwindContextArgs`] wrapper with
/// the given function arguments or scope variables.
///
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
/// use unwind_context::build_unwind_context_data;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _data = build_unwind_context_data!(fn());
///     let _data = build_unwind_context_data!(fn(a, &b, c));
///     let _data = build_unwind_context_data!(fn(a, b.clone(), c));
///     let _data = build_unwind_context_data!(fn(..., c));
///     let _data = build_unwind_context_data!(fn(a, ...));
///     let _data = build_unwind_context_data!(fn(a, ..., c));
///     let _data = build_unwind_context_data!(fn(a, &b, c, "step 1"));
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
/// use unwind_context::build_unwind_context_data;
///
/// fn func(a: u32, b: String, c: bool) {
///     let _data = build_unwind_context_data!(fn func());
///     let _data = build_unwind_context_data!(fn func(a, &b, c));
///     let _data = build_unwind_context_data!(fn func(a, b.clone(), c));
///     let _data = build_unwind_context_data!(fn func(..., c));
///     let _data = build_unwind_context_data!(fn func(a, ...));
///     let _data = build_unwind_context_data!(fn func(a, ..., c));
///     let _data = build_unwind_context_data!(fn func(a, &b, c, "step 1"));
///     let _data = build_unwind_context_data!(fn "func"());
///     let _data = build_unwind_context_data!(fn "mod1::mod2::func"());
///     let _data = build_unwind_context_data!(fn "mod1::mod2::func"(a, &b, c));
///     // ...
/// }
/// ```
///
/// - Create [`UnwindContextArgs`] with the given scope attributes.
///
/// ```rust
/// use unwind_context::build_unwind_context_data;
///
/// fn func(a: u32) {
///     let b = a.to_string();
///     let c = a > 100;
///     let _data = build_unwind_context_data!(a, &b, c);
///     let _data = build_unwind_context_data!(a, b.clone(), c);
///     let _data = build_unwind_context_data!(..., c);
///     let _data = build_unwind_context_data!(a, ...);
///     let _data = build_unwind_context_data!(a, ..., c);
///     let _data = build_unwind_context_data!(a, &b, c, "step 1");
///     // ...
/// }
/// ```
///
/// [`UnwindContextFunc`]: crate::UnwindContextFunc
/// [`UnwindContextArgs`]: crate::UnwindContextArgs
#[macro_export]
macro_rules! build_unwind_context_data {
    ( fn $name:ident ( $( $args:tt )* ) ) => {
        $crate::build_unwind_context_data_impl!( @fn ::core::stringify!($name), $($args)* )
    };
    ( fn $name:literal ( $( $args:tt )* ) ) => {
        $crate::build_unwind_context_data_impl!( @fn $name, $($args)* )
    };
    ( fn ( $( $args:tt )* ) ) => {
        $crate::build_unwind_context_data_impl!( @fn $crate::func_name!(), $($args)* )
    };
    ( $( $vars:tt )* ) => {
        $crate::UnwindContextArgs::new(
            $crate::build_unwind_context_data_impl!( @args $($vars)* )
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! build_unwind_context_data_impl {
    ( @fn $name:expr, $( $args:tt )* ) => {
        $crate::UnwindContextFunc::new(
            $name,
            $crate::build_unwind_context_data_impl!( @args $($args)* )
        )
    };
    ( @args ... $(, $( $args:tt )* )? ) => {
        (
            $crate::UnwindContextArg::new( None, $crate::NonExhaustiveMarker ),
            $crate::build_unwind_context_data_impl!( @args $( $($args)* )? ),
        )
    };
    ( @args $value:literal $(, $( $args:tt )* )? ) => {
        (
            $crate::UnwindContextArg::new( None, $value ),
            $crate::build_unwind_context_data_impl!( @args $( $($args)* )? ),
        )
    };
    ( @args $arg:expr $(, $( $args:tt )* )? ) => {
        (
            $crate::UnwindContextArg::new( Some(::core::stringify!($arg)), $arg ),
            $crate::build_unwind_context_data_impl!( @args $( $($args)* )? ),
        )
    };
    ( @args ) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    use core::fmt::Debug;

    use crate::test_util::buf_fmt;

    #[allow(clippy::similar_names)]
    #[test]
    fn test_unwind_context_data() {
        fn inner_context1(foo: i32, bar: &str) -> impl '_ + Debug {
            build_unwind_context_data!(fn(foo, bar))
        }

        fn inner_context2(foo: i32, bar: &str, _extra_data: ()) -> impl '_ + Debug {
            build_unwind_context_data!(fn(foo, bar, ...))
        }

        let mut buffer = [0; 128];
        let foo = 123;
        let bar = "value";

        let context = build_unwind_context_data!(foo);
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "foo: 123");

        let context = build_unwind_context_data!(foo, 234);
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "foo: 123, 234");

        let context = build_unwind_context_data!(foo, 234, bar);
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "foo: 123, 234, bar: \"value\"");

        let context = build_unwind_context_data!(fn func(foo, 234, bar));
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "fn func(foo: 123, 234, bar: \"value\")");

        let context = build_unwind_context_data!(fn "mod::func"(foo, 234, bar));
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "fn mod::func(foo: 123, 234, bar: \"value\")");

        let context = build_unwind_context_data!(fn func(..., foo, 234, bar));
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "fn func(..., foo: 123, 234, bar: \"value\")");

        let context = build_unwind_context_data!(fn func(foo, ..., 234, bar));
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "fn func(foo: 123, ..., 234, bar: \"value\")");

        let context = build_unwind_context_data!(fn func(foo, 234, bar, ...));
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert_eq!(formatted, "fn func(foo: 123, 234, bar: \"value\", ...)");

        let context = inner_context1(foo, bar);
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert!(formatted.starts_with("fn "));
        assert!(formatted.contains("inner_context1"));
        assert!(formatted.ends_with("(foo: 123, bar: \"value\")"));

        let context = inner_context2(foo, bar, ());
        let formatted = buf_fmt(&mut buffer, format_args!("{context:?}")).unwrap();
        assert!(formatted.starts_with("fn "));
        assert!(formatted.contains("inner_context2"));
        assert!(formatted.ends_with("(foo: 123, bar: \"value\", ...)"));
    }
}
