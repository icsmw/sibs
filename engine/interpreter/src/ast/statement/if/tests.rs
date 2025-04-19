use crate::*;

test_value_expectation!(
    r#if_000,
    Block,
    RtValue::Bool(true),
    "{ if 5 > 1 { true } else { false }; }"
);

test_value_expectation!(
    r#if_001,
    Block,
    RtValue::Bool(true),
    "{ if 5 > 1 { true }; }"
);

test_value_expectation!(r#if_002, Block, RtValue::Void, "{ if 5 > 10 { true }; }");

test_value_expectation!(
    r#if_003,
    Block,
    RtValue::Bool(true),
    "{ if 5 > 10 || 1 > 0 { true }; }"
);
