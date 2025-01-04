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

test_success!(
    closure_declaration_001,
    Block,
    r#"{
        fn sum(a: num, b: num, cb: |n: num|: num) {
            a + b ;
        }
        let cb = |n: num| { n * 2;};
        let a = sum(5, 5, cb);
        a;
    }"#
);

test_success!(
    closure_declaration_002,
    Block,
    r#"{
        fn sum(a: num, b: num, cb: |n: num|: num) {
            a + b + cb(5);
        }
        let cb = |n: num| { n * 2;};
        let a = sum(5, 5, cb);
        a;
    }"#
);
