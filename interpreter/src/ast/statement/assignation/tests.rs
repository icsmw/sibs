use crate::*;

test_value_expectation!(
    assignation_000,
    Block,
    RtValue::Num(5.0),
    "{ let a: num; a = 5; a; }"
);

test_value_expectation!(
    assignation_001,
    Block,
    RtValue::Num(5.0),
    "{ let a: num = 5; a; }"
);

test_value_expectation!(
    assignation_002,
    Block,
    RtValue::Num(5.0),
    "{ let a = 5; a; }"
);

test_value_expectation!(
    assignation_003,
    Block,
    RtValue::Num(5.0),
    "{ let a = 2; a = 5; a; }"
);

test_value_expectation!(
    assignation_004,
    Block,
    RtValue::Bool(true),
    "{ let a = 2; a = 5; if a == 5 { true } else { false } }"
);

test_value_expectation!(
    assignation_005,
    Block,
    RtValue::Num(5.0),
    "{ let a = 2; a = 5; if a == 5 { a } else { 0 } }"
);

test_value_expectation!(
    assignation_006,
    Block,
    RtValue::Bool(true),
    "{ let a = 2; a = 5; a == 5; }"
);

test_value_expectation!(
    assignation_007,
    Block,
    RtValue::Bool(true),
    "{ let a = 2; a = 4; let b = 1; a == 5 || b == 1; }"
);
