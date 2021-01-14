#![deny(rustdoc, missing_docs, warnings, clippy::pedantic)]
#![allow(missing_doc_code_examples)]
#![doc(test(attr(deny(warnings), forbid(unsafe_code))))]
#![no_std]
//! An attribute macro to apply function-like macros.
//! It can apply *multiple* function-like macros that *only* accept an item (do
//! *not* accept other function-like macro calls) to a single item or just
//! improve the *readability* of the code.
//!
//! This crate has *no* dependency so you don't need to worry about compile
//! time.
//!
//! # Examples
//! ```
//! use apply_macro::apply;
//!
//! macro_rules! derive_debug {
//!     {
//!         #[$attr:meta] // will receive `#[apply(derive_clone, derive_partial_eq)]`
//!         $input:item
//!     } => {
//!         #[$attr]
//!         #[derive(Debug)]
//!         $input
//!     };
//! }
//!
//! macro_rules! derive_clone {
//!     {
//!         #[$attr:meta] // will receive `#[apply(derive_partial_eq)]`
//!         $input:item
//!     } => {
//!         #[$attr]
//!         #[derive(Clone)]
//!         $input
//!     };
//! }
//!
//! macro_rules! derive_partial_eq {
//!     ($input:item) => {
//!         #[derive(PartialEq)]
//!         $input
//!     };
//! }
//!
//! #[apply(derive_debug, derive_clone, derive_partial_eq)]
//! struct Num(i32);
//!
//! assert_eq!(Num(-1).clone(), Num(-1));
//! assert_ne!(Num(1), Num(-1));
//!
//! #[apply(derive_debug, derive_clone, derive_partial_eq,)]
//! struct TrailingCommaIsAllowed;
//!
//! assert_eq!(TrailingCommaIsAllowed, TrailingCommaIsAllowed);
//! ```
//!
//! Single macro example:
//! ```
//! use apply_macro::apply;
//!
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
//!
//! #[apply(common_derive,)]
//! struct TrailingCommaIsAllowed;
//!
//! assert_eq!(TrailingCommaIsAllowed, TrailingCommaIsAllowed);
//! ```
//!
//! The `#[apply(common_derive)]` on `Num` expands to:
//! ```
//! # macro_rules! common_derive {
//! #     ($dummy:item) => {};
//! # }
//! common_derive! {
//!     struct Num(i32);
//! }
//! ```
//!
//! Empty argument is allowed (consistent with `#[derive()]`):
//! ```
//! use apply_macro::apply;
//!
//! #[apply()]
//! #[derive()] // consistent
//! # #[allow(unused_attributes, dead_code)]
//! struct EmptyArg;
//! ```
//!
//! Although, as a procedural macro, `#[apply]` can't be banned:
//! ```
//! # use apply_macro::apply;
//! #[apply]
//! # #[allow(dead_code)]
//! struct Oops;
//! ```

use core::iter::once;
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

fn into_tt(tt: impl Into<TokenTree>) -> impl Iterator<Item = TokenTree> {
    once(tt.into())
}

macro_rules! punct {
    [$punct1:literal $($punct:literal)*] => {
        into_tt(Punct::new($punct1, Spacing::Joint))
            $(.chain(into_tt(Punct::new($punct, Spacing::Joint))))*
    };
}

/// The main attribute macro of this crate.
///
/// This macro accepts comma-separated paths to the function-like macros you
/// want to call as arguments. See also [examples in the crate-level
/// documentation] (crate#example).
///
/// ## Limitation
/// **Note that this may be fixed in the future without a major version bump.
/// Do not rely on it.**
///
/// This macro does not check for invalid arguments:
/// ```
/// use apply_macro::apply;
///
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
    if args.is_empty() {
        input
    } else {
        let mut args = args.into_iter();
        let mut result = TokenStream::new();
        for tt in &mut args {
            if let TokenTree::Punct(ref punct) = tt {
                // TODO: https://github.com/rust-lang/rust/pull/78636
                if punct.as_char() == ',' {
                    let args: TokenStream = args.collect();
                    if args.is_empty() {
                        break;
                    }
                    result.extend(
                        punct!['!'].chain(into_tt(Group::new(
                            Delimiter::Brace,
                            punct!['#']
                                .chain(into_tt(Group::new(
                                    Delimiter::Bracket,
                                    punct![':' ':']
                                        .chain(into_tt(Ident::new(
                                            "apply_macro",
                                            Span::call_site(),
                                        )))
                                        .chain(punct![':' ':'])
                                        .chain(into_tt(Ident::new("apply", Span::call_site())))
                                        .chain(into_tt(Group::new(Delimiter::Parenthesis, args)))
                                        .collect(),
                                )))
                                .chain(input)
                                .collect(),
                        ))),
                    );
                    return result;
                }
            }
            result.extend(once(tt));
        }
        result.extend(punct!['!'].chain(into_tt(Group::new(Delimiter::Brace, input))));
        result
    }
}
