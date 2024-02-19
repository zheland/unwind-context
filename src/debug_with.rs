use core::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// An utility wrapper type which is used to forward both [`core::fmt::Debug`]
/// and [`core::fmt::Display`] value implementations to its
/// [`core::fmt::Display`] implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct WithDisplay<T>(
    /// The wrapped value to be formatted with [`core::fmt::Display`] regardless
    /// of whether formatting is invoked with [`core::fmt::Debug`] or
    /// [`core::fmt::Display`] formatter.
    pub T,
);

/// An utility wrapper type which is used to forward both [`core::fmt::Debug`]
/// and [`core::fmt::Display`] value implementations to its [`core::fmt::Debug`]
/// implementation with pretty flag.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct WithPrettyDebug<T>(
    /// The wrapped value to be formatted with pretty
    /// [`core::fmt::Debug`] variant regardless of whether formatting is
    /// invoked with [`core::fmt::Debug`] or [`core::fmt::Display`]
    /// formatter.
    pub T,
);

impl<T> Display for WithDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<T> Debug for WithDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<T> Display for WithPrettyDebug<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self.0)
    }
}

impl<T> Debug for WithPrettyDebug<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:#?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util::buf_fmt;
    use crate::{WithDisplay, WithPrettyDebug};

    #[derive(Clone, Debug)]
    struct Struct {
        _first: u32,
        _second: &'static str,
    }

    #[test]
    fn test_debug_with_display() {
        let mut buffer = [0; 16];
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{}", "foo\nbar")),
            Ok("foo\nbar")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{:?}", "foo\nbar")),
            Ok("\"foo\\nbar\"")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{}", WithDisplay("foo\nbar"))),
            Ok("foo\nbar")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{:?}", WithDisplay("foo\nbar"))),
            Ok("foo\nbar")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{:#?}", WithDisplay("foo\nbar"))),
            Ok("foo\nbar")
        );
    }

    #[test]
    fn test_debug_with_pretty_debug() {
        let value = Struct {
            _first: 1,
            _second: "foo\nbar",
        };
        let mut buffer = [0; 64];
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{value:?}")),
            Ok("Struct { _first: 1, _second: \"foo\\nbar\" }")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{value:#?}")),
            Ok("Struct {\n    _first: 1,\n    _second: \"foo\\nbar\",\n}")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{}", WithPrettyDebug(&value))),
            Ok("Struct {\n    _first: 1,\n    _second: \"foo\\nbar\",\n}")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{:?}", WithPrettyDebug(&value))),
            Ok("Struct {\n    _first: 1,\n    _second: \"foo\\nbar\",\n}")
        );
        assert_eq!(
            buf_fmt(&mut buffer, format_args!("{:#?}", WithPrettyDebug(&value))),
            Ok("Struct {\n    _first: 1,\n    _second: \"foo\\nbar\",\n}")
        );
    }
}
