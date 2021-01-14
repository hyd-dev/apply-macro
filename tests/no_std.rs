#![forbid(unsafe_code)]
#![no_std]

use apply_macro::apply;

macro_rules! common_derive {
    ($input:item) => {
        #[derive(Debug, PartialEq)]
        $input
    };
}

#[apply(common_derive)]
struct Num(i32);

#[test]
fn no_std() {
    assert_eq!(Num(-1), Num(-1));
    assert_ne!(Num(1), Num(-1));
}
