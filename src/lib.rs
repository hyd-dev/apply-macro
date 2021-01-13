#![deny(rustdoc, missing_docs, warnings, clippy::pedantic)]
#![doc(test(attr(deny(warnings))))]
//! An attribute macro to apply function-like macros.
//! It can improve the readability of your code.
//!
//! This crate has *no* dependency so you don't need to worry about compile time.
//!
//! # Example
//! ```
//! # use apply_macro::apply;
//! #
//! macro_rules! common_derive {
//!     ($input:item) => {
//!         #[derive(Debug, PartialEq)]
//!         $input
//!     };
//! }
//!
//! #[apply(common_derive)]
//! struct Num(i32);
//!
//! assert_eq!(Num(-1), Num(-1));
//! assert_ne!(Num(1), Num(-1));
//! ```
//!
//! The `#[apply(common_derive)]` above expands to:
//! ```
//! # macro_rules! common_derive {
//! #     ($dummy:item) => {};
//! # }
//! common_derive! {
//!     struct Num(i32);
//! }
//! ```
#![no_std]

use core::iter::once;
use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream};

/// The main attribute macro of this crate.
///
/// This accepts the path to the function-like macro you want to call as argument.
/// See also [the example in the crate-level documentation](crate#example).
///
/// ## Known issue
/// This macro does not check for invalid arguments:
/// ```
/// # use apply_macro::apply;
/// #
/// macro_rules! derive_debug {
///     ($input:item) => {
///         #[derive(Debug)]
///         $input
///     };
/// }
///
/// #[apply(#[derive(Debug)] struct AnotherStruct; derive_debug)]
/// struct ImplsDebug;
///
/// dbg!(AnotherStruct, ImplsDebug);
/// ```
#[proc_macro_attribute]
pub fn apply(args: TokenStream, input: TokenStream) -> TokenStream {
    args.into_iter()
        .chain(once(Punct::new('!', Spacing::Alone).into()))
        .chain(once(Group::new(Delimiter::Brace, input).into()))
        .collect()
}
