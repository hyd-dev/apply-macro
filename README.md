# apply-macro
A Rust attribute macro to apply function-like macros, used to apply *multiple* function-like macros that *only* accept an item (do *not* accept other function-like macro calls) to a single item or just improve the *readability* of the code.

This crate has *no* dependency so you don't need to worry about compile time.

[![CI](https://github.com/hyd-dev/apply-macro/workflows/CI/badge.svg)](https://github.com/hyd-dev/apply-macro/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/apply-macro.svg)](https://crates.io/crates/apply-macro)

## Example
```rust
use apply_macro::apply;

macro_rules! derive_debug {
    {
        #[$attr:meta] // will receive `#[apply(derive_partial_eq)]`
        $input:item
    } => {
        #[$attr]
        #[derive(Debug)]
        $input
    };
}

macro_rules! derive_partial_eq {
    ($input:item) => {
        #[derive(PartialEq)]
        $input
    };
}

#[apply(derive_debug, derive_partial_eq)]
struct Num(i32);

assert_eq!(Num(-1), Num(-1));
assert_ne!(Num(1), Num(-1));
```

Single macro example:
```rust
use apply_macro::apply;

macro_rules! common_derive {
    ($input:item) => {
        #[derive(Debug, PartialEq)]
        $input
    };
}

#[apply(common_derive)]
struct Num(i32);

assert_eq!(Num(-1), Num(-1));
assert_ne!(Num(1), Num(-1));
```

The `#[apply(common_derive)]` above expands to:
```rust
common_derive! {
    struct Num(i32);
}
```

Check out the [documentation](https://docs.rs/apply-macro) for more examples.

## Supported Rust versions
Latest nightly, beta and stable.

## License
This project is [licensed](./COPYRIGHT.md) under the [Mozilla Public License Version 2.0](./LICENSE.md).

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be licensed as [MPL 2.0](./LICENSE.md), without any additional terms or conditions.
