use core::fmt::{Debug, Formatter, Result as FmtResult};

use crate::{AnsiColorScheme, AnsiColored, DebugAnsiColored, UnwindContextArgs};

/// A structure representing function name and its argument names and values.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnwindContextFunc<Args> {
    /// Function name.
    pub name: &'static str,
    /// Function argument names and values.
    pub args: Args,
}

impl<Args> UnwindContextFunc<Args> {
    /// Create a new `UnwindContextFunc` with the provided name and arguments.
    #[inline]
    pub fn new(name: &'static str, args: Args) -> Self {
        Self { name, args }
    }
}

impl<Args> Debug for UnwindContextFunc<Args>
where
    for<'a> UnwindContextArgs<&'a Args>: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "fn {}({:?})",
            self.name,
            UnwindContextArgs::new(&self.args)
        )?;
        Ok(())
    }
}

impl<Args> DebugAnsiColored for UnwindContextFunc<Args>
where
    for<'a> UnwindContextArgs<&'a Args>: DebugAnsiColored,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        write!(
            f,
            "{}fn {}{}{}({}{:?}{}){}",
            color_scheme.fn_keyword,
            color_scheme.func_name,
            self.name,
            color_scheme.func_braces,
            color_scheme.default,
            AnsiColored::new(UnwindContextArgs::new(&self.args), color_scheme),
            color_scheme.func_braces,
            color_scheme.default,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::fmt::Error as FmtError;

    use crate::test_common::{arg, TEST_ANSI_COLOR_SCHEME};
    use crate::test_util::debug_fmt;
    use crate::{AnsiColored, UnwindContextFunc};

    #[test]
    fn test_func_fmt() {
        let mut buffer = [0; 64];

        assert_eq!(
            debug_fmt(&mut buffer, &UnwindContextFunc::new("foo", ())),
            Ok("fn foo()")
        );
        assert_eq!(
            debug_fmt(
                &mut buffer,
                &UnwindContextFunc::new("foo", (arg(Some("bar"), 1), ()))
            ),
            Ok("fn foo(bar: 1)")
        );
        assert_eq!(
            debug_fmt(
                &mut buffer,
                &UnwindContextFunc::new("foo", (arg(Some("bar"), 1), (arg(Some("baz"), 2), ())))
            ),
            Ok("fn foo(bar: 1, baz: 2)")
        );
    }

    #[test]
    fn test_func_colored_fmt() {
        let mut buffer = [0; 128];

        assert_eq!(
            debug_fmt(
                &mut buffer,
                &AnsiColored::new(
                    UnwindContextFunc::new("foo", (arg(Some("bar"), 1), (arg(Some("baz"), 2), ()))),
                    &TEST_ANSI_COLOR_SCHEME
                )
            ),
            Ok(concat!(
                "{FN}fn ",
                "{FN_NAME}foo",
                "{FN_BRACE}(",
                "{DEF}bar: ",
                "{NUM}1",
                "{DEF}, baz: ",
                "{NUM}2",
                "{DEF}",
                "{FN_BRACE}",
                ")",
                "{DEF}"
            ))
        );
    }

    #[test]
    fn test_func_failed_fmt() {
        let func = UnwindContextFunc::new("foo", (arg(Some("foo"), 1), (arg(Some("bar"), 2), ())));

        let mut buffer = [0; 64];
        let len = debug_fmt(&mut buffer, &func).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &func), Err(FmtError));
        }
    }

    #[test]
    fn test_func_failed_colored_fmt() {
        let func = AnsiColored::new(
            UnwindContextFunc::new("foo", (arg(Some("foo"), 1), (arg(Some("bar"), 2), ()))),
            &TEST_ANSI_COLOR_SCHEME,
        );

        let mut buffer = [0; 128];
        let len = debug_fmt(&mut buffer, &func).unwrap().len();
        for len in 0..len {
            assert_eq!(debug_fmt(&mut buffer[0..len], &func), Err(FmtError));
        }
    }
}
