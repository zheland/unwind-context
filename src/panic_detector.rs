/// An utility trait which is used to detect panic.
pub trait PanicDetector {
    /// Determines whether the current thread is unwinding because of panic.
    ///
    /// # Panics
    ///
    /// Implementations should generally avoid [`panic!`]ing, because
    /// `is_panicking()` may itself be called during unwinding due to a
    /// panic, and if the `is_panicking()` panics in that situation (a “double
    /// panic”), this will likely abort the program.
    fn is_panicking(&self) -> bool;
}

/// A default [`PanicDetector`] for a crates compiled with the Rust standard
/// library.
///
/// It uses `std::thread::panicking()` to detect whether the current thread is
/// unwinding because of panic.
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StdPanicDetector;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl PanicDetector for StdPanicDetector {
    #[inline]
    fn is_panicking(&self) -> bool {
        std::thread::panicking()
    }
}
