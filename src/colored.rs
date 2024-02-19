use core::fmt::{Debug, Formatter, Result as FmtResult};

use crate::AnsiColorScheme;

/// An utility alternative [`core::fmt::Debug`] trait which can used for colored
/// context formatting.
pub trait DebugAnsiColored {
    /// Formats the value using with colorization and a given
    /// [`AnsiColorScheme`].
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult;
}

/// An utility wrapper type is used to forward value [`core::fmt::Debug`]
/// implementation to [`DebugAnsiColored`] implementation with a given
/// [`AnsiColorScheme`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AnsiColored<T> {
    /// The wrapped value to be formatted with [`DebugAnsiColored`].
    pub value: T,
    /// Selected color scheme.
    pub color_scheme: &'static AnsiColorScheme,
}

impl<T> AnsiColored<T> {
    /// Wraps a given `T` so its [`core::fmt::Debug`] implementation will
    /// forward to `DebugAnsiColored` with a given color scheme.
    #[inline]
    pub fn new(value: T, color_scheme: &'static AnsiColorScheme) -> Self {
        Self {
            value,
            color_scheme,
        }
    }
}

impl<T> Debug for AnsiColored<T>
where
    T: DebugAnsiColored,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        DebugAnsiColored::fmt_colored(&self.value, f, self.color_scheme)
    }
}

impl<T> DebugAnsiColored for &T
where
    T: DebugAnsiColored,
{
    #[inline]
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult {
        DebugAnsiColored::fmt_colored(&**self, f, color_scheme)
    }
}
