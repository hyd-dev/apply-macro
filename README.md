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

Single macro (`thread_local!`) example:
```rust
use apply_macro::apply;
use std::cell::Cell;

#[apply(thread_local)]
static TLS: Cell<i32> = 1.into();

TLS.with(|tls| assert_eq!(tls.replace(-1), 1));
TLS.with(|tls| assert_eq!(tls.get(), -1));
```

The `#[apply(thread_local)]` above expands to:
```rust
thread_local! {
    static TLS: Cell<i32> = 1.into();
}
```

Check out the [documentation](https://docs.rs/apply-macro) for more examples.

## Supported Rust versions
Latest nightly, beta and stable.

## License
This project is [licensed](./COPYRIGHT.md) under the [Mozilla Public License Version 2.0](./LICENSE.md).
