use core::fmt::{Debug, Formatter, Result as FmtResult};

/// A marker type which is used in arguments list to indicate that there are
/// some other arguments that are omitted.
///
/// It is formatted as a `...` placeholder.
///
/// This type is not intended to be used directly. Consider using macros like
/// [`build_unwind_context_data`] or [`unwind_context`] instead.
///
/// # Examples
///
/// ```rust
/// let arg = unwind_context::UnwindContextArg::new(None, unwind_context::NonExhaustiveMarker);
/// ```
///
/// [`build_unwind_context_data`]: crate::build_unwind_context_data
/// [`unwind_context`]: crate::unwind_context
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct NonExhaustiveMarker;

impl Debug for NonExhaustiveMarker {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("...")
    }
}
