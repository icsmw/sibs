use crate::*;

test_success!(
    closure_declaration_000,
    Block,
    r#"{
        fn test(a: num, cb: |n: num|: num, c: num) {
            let b = 5;
            cb(a);
        }
    }"#
);
