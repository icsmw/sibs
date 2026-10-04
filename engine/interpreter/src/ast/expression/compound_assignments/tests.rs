use crate::*;
use std::path::PathBuf;

test_value_expectation!(
    compound_assignments_000,
    Block,
    RtValue::Num(55.0),
    r#"{
        let sum = 0;
        for(el, n) in 0..10 {
            sum += el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_001,
    Block,
    RtValue::Num(0.0),
    r#"{
        let sum = 55;
        for(el, n) in 0..10 {
            sum -= el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_002,
    Block,
    RtValue::Num(120.0),
    r#"{
        let sum = 1;
        for(el, n) in 0..5 {
            if el != 0 {
                sum *= el;
            }
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_004,
    Block,
    RtValue::Num(5.0),
    r#"{
        let sum = 10;
        for(el, n) in 1..2 {
            sum /= el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_005,
    Block,
    RtValue::Num(20.0),
    r#"{
        let sum = 10;
        sum += 10;
    }"#
);

test_value_expectation!(
    compound_assignments_006,
    Block,
    RtValue::Str(String::from("HelloHelloHelloHelloHello")),
    r#"{
        let full = "";
        for(el, n) in 0..4 {
            full += "Hello";
        };
        full;
    }"#
);

test_value_expectation!(
    compound_assignments_007,
    Block,
    RtValue::Str(String::from("value: 2")),
    r#"{
        let full = "value: ";
        let n = 2;
        full += '{n}';
        full;
    }"#
);

#[test]
fn path_plus_equal_joins_components_without_changing_operands() {
    let left = RtValue::PathBuf(PathBuf::from("workspace"));
    let right = RtValue::PathBuf(PathBuf::from("artifacts").join("release"));

    let result =
        super::apply_operator(&left, &CompoundAssignmentsOperator::PlusEqual, &right).unwrap();

    assert_eq!(
        result,
        RtValue::PathBuf(["workspace", "artifacts", "release"].iter().collect())
    );
    assert_eq!(left, RtValue::PathBuf(PathBuf::from("workspace")));
    assert_eq!(
        right,
        RtValue::PathBuf(["artifacts", "release"].iter().collect())
    );
}

#[test]
fn path_plus_equal_with_an_absolute_path_replaces_the_left_path() {
    let left = RtValue::PathBuf(PathBuf::from("workspace"));
    let right = RtValue::PathBuf(std::env::current_dir().unwrap().join("artifacts"));

    assert_eq!(
        super::apply_operator(&left, &CompoundAssignmentsOperator::PlusEqual, &right).unwrap(),
        right
    );
}

#[test]
fn paths_reject_other_compound_operators() {
    let left = RtValue::PathBuf(PathBuf::from("workspace"));
    let right = RtValue::PathBuf(PathBuf::from("artifacts"));

    for operator in [
        CompoundAssignmentsOperator::MinusEqual,
        CompoundAssignmentsOperator::SlashEqual,
        CompoundAssignmentsOperator::StarEqual,
    ] {
        assert!(matches!(
            super::apply_operator(&left, &operator, &right),
            Err(E::NotApplicableToTypeOperation)
        ));
    }
}
