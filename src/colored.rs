use core::fmt::{Debug, Formatter, Result as FmtResult};

use crate::AnsiColorScheme;

/// An utility alternative [`core::fmt::Debug`] trait which can used for colored
/// context formatting.
///
/// This trait is not intended to be used directly. It is used for coloring
/// functions and arguments data returned by macros like
/// [`build_unwind_context_data`] or [`unwind_context`] instead.
///
/// # Examples
///
/// ```rust
/// use unwind_context::{are_colors_enabled, unwind_context, DebugAnsiColored};
///
/// fn fmt_example(writer: core::fmt::Write, value: impl Debug + DebugAnsiColored) {
///     if are_colors_enabled() {
///         let _ = writeln!(
///             "{:?}",
///             AnsiColored::new(value, &unwind_context::DEFAULT_DEFAULT_COLOR_SCHEME)
///         );
///     } else {
///         let _ = writeln!("{value:?}");
///     }
/// }
/// ```
///
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
/// [`unwind_context`]: crate::unwind_context
pub trait DebugAnsiColored {
    /// Formats the value using with colorization and a given
    /// [`AnsiColorScheme`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the value formatting fails.
    fn fmt_colored(
        &self,
        f: &mut Formatter<'_>,
        color_scheme: &'static AnsiColorScheme,
    ) -> FmtResult;
}

/// An utility wrapper type is used to forward value [`core::fmt::Debug`]
/// implementation to [`DebugAnsiColored`] implementation with a given
/// [`AnsiColorScheme`].
///
/// This type is not intended to be used directly. Consider using macros like
/// [`unwind_context`], [`unwind_context_with_io`] or
/// [`unwind_context_with_fmt`] instead.
///
/// [`unwind_context`]: crate::unwind_context
/// [`unwind_context_with_io`]: crate::unwind_context_with_io
/// [`unwind_context_with_fmt`]: crate::unwind_context_with_fmt
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// let arg = unwind_context::AnsiColored::new(
    ///     unwind_context::UnwindContextArg::new(Some("foo"), 123),
    ///     &unwind_context::DEFAULT_DEFAULT_COLOR_SCHEME,
    /// );
    /// ```
    ///
    /// [`build_unwind_context_data`]: crate::build_unwind_context_data
    /// [`unwind_context`]: crate::unwind_context
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
