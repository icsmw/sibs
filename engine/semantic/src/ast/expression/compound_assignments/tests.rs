use crate::*;

test_success!(
    compound_assignments_000,
    Block,
    r#"{
        let a = 1;
        a += 1;
    }"#
);

test_success!(
    compound_assignments_001,
    Block,
    r#"{
        let a = 1;
        let b = 2;
        a += b;
    }"#
);

test_success!(
    compound_assignments_002,
    Block,
    r#"{
        let a = 1;
        let b: num = 1;
        a += b;
    }"#
);

test_success!(
    compound_assignments_003,
    Block,
    r#"{
        let a = 1;
        let b = 1;
        a -= b;
    }"#
);

test_success!(
    compound_assignments_004,
    Block,
    r#"{
        let a = 1;
        let b = 1;
        a *= b;
    }"#
);

test_success!(
    compound_assignments_005,
    Block,
    r#"{
        let a = 1;
        let b = 1;
        a /= b;
    }"#
);

test_fail!(
    compound_assignments_000,
    Block,
    r#"{
        let a = 1;
        let b = "2";
        a += b;
    }"#
);

test_fail!(
    compound_assignments_001,
    Block,
    r#"{
        let a = 1;
        let b;
        a += b;
    }"#
);

test_fail!(
    compound_assignments_002,
    Block,
    r#"{
        let a = 1;
        let b: num;
        a += b;
    }"#
);

test_fail!(
    compound_assignments_003,
    Block,
    r#"{
        let a = "1";
        let b = 1;
        a += b;
    }"#
);
