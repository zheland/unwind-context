use core::fmt::{Debug, Formatter, Result as FmtResult};

use crate::{AnsiColorScheme, AnsiColored, DebugAnsiColored, UnwindContextArg};

/// A structure representing function argument names and their values.
///
/// This type is not intended to be used directly. Consider using macros like
/// [`build_unwind_context_data`] or [`unwind_context`] instead.
///
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
/// [`unwind_context`]: crate::unwind_context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnwindContextArgs<Params>(
    /// Function argument names and values in cons-like list representation.
    pub Params,
);

impl<Params> UnwindContextArgs<Params> {
    /// Create a new `UnwindContextArgs` with the provided parameters.
    ///
    /// Parameters are required to be represented as a recursive tuple list like
    /// `(A, (B, (C, (D, ()))))` in order to be formatted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use unwind_context::{UnwindContextArg, UnwindContextArgs};
    ///
    /// let args0 = UnwindContextArgs::new(());
    ///
    /// let args1 = UnwindContextArgs::new((UnwindContextArg::new(Some("first"), 123), ()));
    ///
    /// let args3 = UnwindContextArgs::new((
    ///     UnwindContextArg::new(Some("first"), 123),
    ///     (
    ///         UnwindContextArg::new(Some("second"), "foo"),
    ///         (UnwindContextArg::new(Some("third"), true), ()),
    ///     ),
    /// ));
    /// ```
    #[inline]
    pub fn new(args: Params) -> Self {
        Self(args)
    }
}

impl Debug for UnwindContextArgs<()> {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

impl Debug for UnwindContextArgs<&()> {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
        Ok(())
    }
}

impl DebugAnsiColored for UnwindContextArgs<()> {
    #[inline]
    fn fmt_colored(&self, _: &mut Formatter<'_>, _: &'static AnsiColorScheme) -> FmtResult {
        Ok(())
    }
}

impl DebugAnsiColored for UnwindContextArgs<&()> {
    #[inline]
    fn fmt_colored(&self, _: &mut Formatter<'_>, _: &'static AnsiColorScheme) -> FmtResult {
        Ok(())
    }
}

impl<First, Rest> Debug for UnwindContextArgs<(First, Rest)>
where
    for<'a> UnwindContextArgs<&'a (First, Rest)>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&UnwindContextArgs(&self.0), f)?;
        Ok(())
    }
}

impl<First, Rest> DebugAnsiColored for UnwindContextArgs<(First, Rest)>
where
    for<'a> UnwindContextArgs<&'a (First, Rest)>: DebugAnsiColored,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        DebugAnsiColored::fmt_colored(&UnwindContextArgs(&self.0), f, color_scheme)?;
        Ok(())
    }
}

impl<First> Debug for UnwindContextArgs<&(UnwindContextArg<First>, ())>
where
    First: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0 .0, f)?;
        Ok(())
    }
}

impl<First> DebugAnsiColored for UnwindContextArgs<&(UnwindContextArg<First>, ())>
where
    First: Debug,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        DebugAnsiColored::fmt_colored(&self.0 .0, f, color_scheme)?;
        Ok(())
    }
}

impl<'a, First, Second, Rest> Debug
    for UnwindContextArgs<&'a (UnwindContextArg<First>, (Second, Rest))>
where
    First: Debug,
    UnwindContextArgs<&'a (Second, Rest)>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}, {:?}", self.0 .0, UnwindContextArgs(&self.0 .1))?;
        Ok(())
    }
}

impl<'a, First, Second, Rest> DebugAnsiColored
    for UnwindContextArgs<&'a (UnwindContextArg<First>, (Second, Rest))>
where
    First: Debug,
    UnwindContextArgs<&'a (Second, Rest)>: DebugAnsiColored,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        write!(
            f,
            "{:?}, {:?}",
            AnsiColored::new(&self.0 .0, color_scheme),
            AnsiColored::new(UnwindContextArgs(&self.0 .1), color_scheme)
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Error as FmtError;

    use crate::test_common::{arg, args, colored_args};
    use crate::test_util::debug_fmt;

    #[test]
    fn test_args_fmt() {
        let mut buffer = [0; 64];

        assert_eq!(debug_fmt(&mut buffer, &args(())), Ok(""));
        assert_eq!(debug_fmt(&mut buffer, &args(&())), Ok(""));

        assert_eq!(
            debug_fmt(&mut buffer, &args((arg(Some("foo"), 1), ()))),
            Ok("foo: 1")
        );
        assert_eq!(
            debug_fmt(&mut buffer, &args(&(arg(Some("foo"), 1), ()))),
            Ok("foo: 1")
        );

        assert_eq!(
            debug_fmt(
                &mut buffer,
                &args(&(arg(Some("foo"), 1), (arg(Some("bar"), 2), ())))
            ),
            Ok("foo: 1, bar: 2")
        );

        assert_eq!(
            debug_fmt(
                &mut buffer,
                &args(&(
                    arg(Some("foo"), 1),
                    (arg(Some("bar"), 2), (arg(Some("baz"), 3), ()))
                ))
            ),
            Ok("foo: 1, bar: 2, baz: 3")
        );

        assert_eq!(
            debug_fmt(
                &mut buffer,
                &args(&(
                    arg(Some("foo"), 1),
                    (arg(Some("bar"), 2), (arg(None, 3), ()))
                ))
            ),
            Ok("foo: 1, bar: 2, 3")
        );
    }

    #[test]
    fn test_args_colored_fmt() {
        let mut buffer = [0; 64];

        assert_eq!(debug_fmt(&mut buffer, &colored_args(())), Ok(""));
        assert_eq!(debug_fmt(&mut buffer, &colored_args(&())), Ok(""));

        assert_eq!(
            debug_fmt(&mut buffer, &colored_args((arg(Some("foo"), 1), ()))),
            Ok("foo: {NUM}1{DEF}")
        );

        assert_eq!(
            debug_fmt(
                &mut buffer,
                &colored_args(&(
                    arg(Some("foo"), 1),
                    (arg(Some("bar"), 2), (arg(None, 3), ()))
                ))
            ),
            Ok("foo: {NUM}1{DEF}, bar: {NUM}2{DEF}, {NUM}3{DEF}")
        );
    }

    #[test]
    fn test_args_failed_fmt() {
        let args = args((arg(Some("foo"), 1), (arg(Some("bar"), 2), ())));

        let mut buffer = [0; 64];
        let len = debug_fmt(&mut buffer, &args).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &args), Err(FmtError));
        }
    }

    #[test]
    fn test_args_failed_colored_fmt() {
        let args = colored_args((arg(Some("foo"), 1), (arg(Some("bar"), 2), ())));

        let mut buffer = [0; 64];
        let len = debug_fmt(&mut buffer, &args).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &args), Err(FmtError));
        }
    }
}
