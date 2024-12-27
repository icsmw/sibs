use crate::*;

test_value_expectation!(binary_exp_000, Block, RtValue::Num(10.0), "{ 5 + 5; }");
test_value_expectation!(binary_exp_001, Block, RtValue::Num(10.0), "{ 5 * 2; }");
test_value_expectation!(binary_exp_002, Block, RtValue::Num(10.0), "{ 20 / 2; }");
test_value_expectation!(binary_exp_003, Block, RtValue::Num(10.0), "{ 15 - 5; }");
test_value_expectation!(binary_exp_004, Block, RtValue::Num(10.0), "{ 2 + 3 + 5; }");
test_value_expectation!(
    binary_exp_005,
    Block,
    RtValue::Num(10.0),
    "{ 2 + 3 + 3 + 2; }"
);
test_value_expectation!(
    binary_exp_006,
    Block,
    RtValue::Num(10.0),
    "{ 5 + (3 + 2); }"
);
test_value_expectation!(
    binary_exp_007,
    Block,
    RtValue::Num(10.0),
    "{ (2 + 3) + (3 + 2); }"
);
test_value_expectation!(
    binary_exp_008,
    Block,
    RtValue::Num(10.0),
    "{ (2 + 3) * 2; }"
);
test_value_expectation!(
    binary_exp_009,
    Block,
    RtValue::Num(10.0),
    "{ 2 + 2 * 3 + 2; }"
);

test_value_expectation!(
    binary_exp_010,
    Block,
    RtValue::Num(10.0),
    "{ 2 + 2 * 3 * 2 / 2 + 2; }"
);

// (2 + 2) * (3 + 2);
// 2 + 2 * 3 + 2;
