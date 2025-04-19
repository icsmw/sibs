use crate::*;

test_success!(
    closure_000,
    Block,
    r#"{
        let cb = |a: num| { a + a; };
        let cb = |a: num, b: str| { a + a; };
        let cb = |a: Vec<num>, b: num| { a[b]; };
    }"#
);
