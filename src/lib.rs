#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

//! The `unwind-context` crate makes debugging panics easier
//! by adding a colored panic context with a simple macro.
#![doc = ""]
#![doc = include_str!("../examples/demo.html")]
#![doc = ""]
//! # Introduction
//!
//! In Rust, panics are typically used when an
//! [unrecoverable](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
//! error occurs or when writing examples, prototype code, or tests.
//!
//! However, it can be difficult to pinpoint the exact cause of a panic,
//! especially if it happens deep in the code or within a loop. While adding
//! logs can help, this may lead to a large number of log entries, making it
//! challenging to identify which ones are related to the panic.
//!
//! # About
//!
//! The goal of this crate is to make the panic context addition simple, and the
//! context itself detailed enough, and easy to read. Accordingly, it also makes
//! it easier to add context to assertions in your tests. This crate provides
//! [`unwind_context`] and [`debug_unwind_context`] macros and some other
//! auxiliary types, traits, functions, and macros that help you define function
//! or scope context and write it to [`std::io::stderr`] or another
//! writeable target if panic occurs. If panic occurs, the context will be
//! written in "reverse" chronological order during the unwinding process.
//!
//! This library adds very little overhead to compiled functions unless they are
//! panicked:
//! - First, it constructs a structure containing the context data, code
//!   location, writer, and color scheme on the stack. It also stores a
//!   reference to the custom panic detector, if specified.
//! - And when this "context scope guard" structure is dropped, its destructor
//!   checks for [`std::thread::panicking`] and calls the cold print function if
//!   panic has been detected.
//!
//! This crate is intended for diagnostic use. The exact contents and format of
//! the messages printed on panic are not specified, other than being a clear
//! and compact description.
//!
//! Note that the context will only be printed if the
//! [`panic`](https://doc.rust-lang.org/cargo/reference/profiles.html#panic)
//! setting is set to `unwind`, which is the default for both
//! [`dev`](https://doc.rust-lang.org/cargo/reference/profiles.html#dev)
//! and
//! [`release`](https://doc.rust-lang.org/cargo/reference/profiles.html#release)
//! profiles.
//!
//! # Usage
//!
//! First, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! unwind-context = "0.2.2"
//! ```
//!
//! Then, add the macro call with the given function arguments or scope
//! arguments to the beginning of the functions to be tracked and bind the
//! result to some scope variable (otherwise the unwind context scope guard will
//! be immediately dropped):
#![cfg_attr(feature = "std", doc = "```rust")]
#![cfg_attr(not(feature = "std"), doc = "```rust,compile_fail")]
//! use unwind_context::unwind_context;
//!
//! fn func1(a: u32, b: &str, c: bool) {
//!     let _ctx = unwind_context!(fn(a, b, c));
//!     // ...
//!     for i in 0..10 {
//!         let _ctx = unwind_context!(i);
//!         // ...
//!     }
//!     // ...
//! }
#![doc = "```"]
#![doc = ""]
//! With `unwind_context!(a, b, c)` syntax, it will print code location,
//! given argument names (stringified expressions), and values on unwind,
//! whereas with `unwind_context!(fn(a, b, c))` it will also print function
//! names as well. Note that it uses the [`core::fmt::Debug`] representation. If
//! you want to use the [`core::fmt::Display`] representation, you can use the
//! [`WithDisplay`] wrapper.
//!
//! You can use the [`set_colors_enabled`] function to unconditionally enable
//! the 16-ANSI-color colorization. If you want to enable colorization only if
//! supported by the terminal, you can use the [`enable_colors_if_supported`]
//! function, which will require enabling the
//! [`detect-color-support`](#feature-flags) feature flag:
//! ```toml
//! [dependencies.unwind-context]
//! version = "0.2.2"
//! features = [ "detect-color-support" ]
//! ```
#![cfg_attr(feature = "detect-color-support", doc = "```rust")]
#![cfg_attr(not(feature = "detect-color-support"), doc = "```rust,compile_fail")]
//! # /*
//! fn main() {
//! # */
//!     unwind_context::enable_colors_if_supported();
//! #   test();
//!     // ...
//! # /*
//! }
//!
//! # */
//! # /*
//! #[test]
//! # */
//! fn test() {
//!     unwind_context::enable_colors_if_supported()
//!     // ...
//! }
#![doc = "```"]
#![doc = ""]
//! If you want to specify a custom color scheme, you can use the
//! [`set_default_color_scheme`] function.
//! Also, colorization can be customized separately for each context scope guard
//! with the [`unwind_context_with_io`] and [`unwind_context_with_fmt`] macros.
//!
//! This crate depends on the standard library by default that is needed to
//! write to [`std::io::stderr`] and to detect panicking using
//! [`std::thread::panicking`]. To use this crate in a `#![no_std]` context with
//! your custom [`core::fmt::Write`] writer and custom [`PanicDetector`], use
//! `default-features = false` in your `Cargo.toml` as shown below:
//! ```toml
//! [dependencies.unwind-context]
//! version = "0.2.2"
//! default-features = false
//! ```
//!
//! # Examples
//!
//! The following crate example:
#![cfg_attr(feature = "detect-color-support", doc = "```rust,should_panic")]
#![cfg_attr(not(feature = "detect-color-support"), doc = "```rust,compile_fail")]
#![doc = include_str!("../examples/demo.rs")]
#![doc = "```"]
//! will output:
#![doc = ""]
#![doc = include_str!("../examples/demo.html")]
#![doc = ""]
//! # Macro expansion
//!
//! The following function:
#![cfg_attr(feature = "std", doc = "```rust")]
#![cfg_attr(not(feature = "std"), doc = "```rust,compile_fail")]
//! use unwind_context::unwind_context;
//!
//! fn foo(a: &str, b: Vec<u8>, c: bool, d: String) {
//!     let _ctx = unwind_context!(fn(a, &b, ..., d.clone()));
//!     // ...
//!     for i in 0..10 {
//!         let _ctx = unwind_context!(i);
//!         // ...
//!     }
//! }
#![doc = "```"]
//! will partially expand into:
#![cfg_attr(feature = "std", doc = "```rust")]
#![cfg_attr(not(feature = "std"), doc = "```rust,compile_fail")]
//! fn foo(a: u32, b: Vec<u8>, c: bool, d: String) {
//!     let _ctx = unwind_context::UnwindContextWithIo::new(
//!         unwind_context::UnwindContextFunc::new(
//!             {
//!                 struct Item;
//!                 let module_path = ::core::module_path!();
//!                 let item_type_name = ::core::any::type_name::<Item>();
//!                 unwind_context::func_name_from_item_type_name(
//!                     module_path, item_type_name
//!                 )
//!             },
//!             (
//!                 unwind_context::UnwindContextArg::new(Some("a"), a),
//!                 (
//!                     unwind_context::UnwindContextArg::new(Some("&b"), &b),
//!                     (
//!                         unwind_context::UnwindContextArg::new(
//!                             None,
//!                             unwind_context::NonExhaustiveMarker,
//!                         ),
//!                         (
//!                             unwind_context::UnwindContextArg::new(
//!                                 Some("d.clone()"), d.clone()
//!                                ),
//!                             (),
//!                         ),
//!                     ),
//!                 ),
//!             ),
//!         ),
//!         ::std::io::stderr(),
//!         unwind_context::StdPanicDetector,
//!         unwind_context::get_default_color_scheme_if_enabled(),
//!     );
//!     // ...
//!     for i in 0..10 {
//!         let _ctx = unwind_context::UnwindContextWithIo::new(
//!             unwind_context::UnwindContextArgs::new((
//!                 unwind_context::UnwindContextArg::new(Some("i"), i),
//!                 (),
//!             )),
//!             ::std::io::stderr(),
//!             unwind_context::StdPanicDetector,
//!             unwind_context::get_default_color_scheme_if_enabled(),
//!         );
//!         // ...
//!     }
//! }
#![doc = "```"]
#![doc = ""]
//! # Feature Flags
//!
//! - `std` (enabled by default): Enables [`UnwindContextWithIo`] structure,
//!   [`unwind_context`], [`debug_unwind_context`], [`unwind_context_with_io`],
//!   and [`debug_unwind_context_with_io`] macros.
//! - `detect-color-support`: Enables [`enable_colors_if_supported`] function
//!   and [`supports-color`] optional dependency.
//! - `custom-default-colors`: Enables [`set_default_color_scheme`] function and
//!   [`atomic_ref`] optional dependency.
//!
//! # Similar crates
//!
//! - [`scopeguard`] allows you to run any code at the end of a scope. It has
//!   both success and unwind guard variants, and it doesn't require panic hook
//!   modification.
//! - [`panic-context`] allows you to specify and modify panic context using a
//!   custom panic hook. It provides more fine-grained control over the output.
//!   However, it implicitly modifies the panic hook using a mutex for a
//!   one-time thread local initialization and doesn’t add any automatic context
//!   or colorization.
//! - [`econtext`] allows you to specify panic context and automatically adds
//!   some context including function name and location. However, it requires
//!   panic hook modification via the init function and uses dynamic dispatch
//!   and some unsafe code.
//!
//! # License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! # Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any
//! additional terms or conditions.
//!
//! [`supports-color`]: https://crates.io/crates/supports-color
//! [`atomic_ref`]: https://crates.io/crates/atomic_ref
//! [`scopeguard`]: https://crates.io/crates/scopeguard
//! [`panic-context`]: https://crates.io/crates/panic-context
//! [`econtext`]: https://crates.io/crates/econtext

#[cfg(feature = "std")]
extern crate std;

#[cfg(test)]
use version_sync as _; // Used in integration tests.

mod arg;
mod args;
mod color_scheme;
mod colored;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod context;
mod context_data;
mod context_with_fmt;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
mod context_with_io;
mod debug_with;
mod func;
mod func_name;
mod non_exhaustive;
mod panic_detector;
mod set_colors;
#[cfg(test)]
mod test_common;
#[cfg(test)]
mod test_util;
mod util_macros;

pub use arg::*;
pub use args::*;
pub use color_scheme::*;
pub use colored::*;
pub use context_with_fmt::*;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use context_with_io::*;
pub use debug_with::*;
pub use func::*;
pub use func_name::*;
pub use non_exhaustive::*;
pub use panic_detector::*;
pub use set_colors::*;
