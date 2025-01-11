use crate::*;

test_success!(
    function_declaration_000,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == "one" {
            } if b == 1 {
            } if c == true {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_001,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == "one" {
            } if b == 1 {
            } if c == false {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_002,
    Block,
    r#"{
        fn test(a: str | num, b: num, c: bool) {
            if a == "one" || a == 2 {
            } if b == 1 {
            } if c == false {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_003,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool, cb: |n: num|: num) {
            a;
        }
    }"#
);

test_fail!(
    function_declaration_000,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == 1 {
            } if b == "one" {
            } if c == 12 {
            } else {
            };
        }
    }"#
);

test_fail!(
    function_declaration_001,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            a = 2;
        }
    }"#
);
