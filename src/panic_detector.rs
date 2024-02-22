/// An utility trait which is used to detect panic.
///
/// # Examples
///
/// ```rust
/// use core::sync::atomic::{self, AtomicBool};
///
/// use unwind_context::unwind_context_with_fmt;
///
/// #[derive(Copy, Clone, Debug)]
/// pub struct NoStdPanicFlag<'a>(&'a AtomicBool);
///
/// impl unwind_context::PanicDetector for NoStdPanicFlag<'_> {
///     fn is_panicking(&self) -> bool {
///         self.0.load(atomic::Ordering::Relaxed)
///     }
/// }
///
/// fn func(foo: u32, bar: &str, writer: &mut String, panic_flag: NoStdPanicFlag<'_>) {
///     let ctx =
///         unwind_context_with_fmt!((foo, bar), writer = writer, panic_detector = panic_flag,);
///     // ...
/// }
/// ```

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
///
/// # Examples
///
/// ```rust
/// use unwind_context::unwind_context_with_fmt;
///
/// fn func(foo: u32, bar: &str, writer: &mut String) {
///     let ctx = unwind_context_with_fmt!(
///         (foo, bar),
///         writer = writer,
///         panic_detector = unwind_context::StdPanicDetector,
///     );
///     // ...
/// }
/// ```
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
