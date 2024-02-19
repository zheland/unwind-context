use core::fmt::{Debug, Formatter, Result as FmtResult};

/// A marker type which is used in arguments list to indicate that there are
/// some other arguments that are omitted. It is formatted as a `...`
/// placeholder.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct NonExhaustiveMarker;

impl Debug for NonExhaustiveMarker {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("...")
    }
}
