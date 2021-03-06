#![allow(renamed_and_removed_lints)] // E0710
#![deny(warnings, missing_docs, rustdoc, clippy::pedantic)]
#![forbid(unsafe_code)]
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
//! Single macro (`thread_local!`) example:
//! ```
//! use apply_macro::apply;
//! use std::cell::Cell;
//!
//! #[apply(thread_local)]
//! static TLS: Cell<i32> = 1.into();
//!
//! TLS.with(|tls| assert_eq!(tls.replace(-1), 1));
//! TLS.with(|tls| assert_eq!(tls.get(), -1));
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
//! #[apply] // same as `#[apply()]`
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
/// documentation](crate#example).
///
/// ## Limitation
/// This macro does not validate its arguments:
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
                if *punct == ',' {
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
