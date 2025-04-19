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
test_value_expectation!(binary_exp_011, Block, RtValue::Num(15.0), "{ 5 + 5 * 2; }");
test_value_expectation!(
    binary_exp_012,
    Block,
    RtValue::Num(18.0),
    "{ (3 + 3) * 3; }"
);
test_value_expectation!(binary_exp_013, Block, RtValue::Num(2.0), "{ 8 - 6; }");
test_value_expectation!(binary_exp_014, Block, RtValue::Num(16.0), "{ 4 * 4; }");
test_value_expectation!(binary_exp_015, Block, RtValue::Num(3.0), "{ 9 / 3; }");
test_value_expectation!(binary_exp_016, Block, RtValue::Num(8.0), "{ 10 - 2; }");
test_value_expectation!(
    binary_exp_017,
    Block,
    RtValue::Num(30.0),
    "{ (5 + 5) * (2 + 1); }"
);
test_value_expectation!(binary_exp_018, Block, RtValue::Num(7.0), "{ 14 / 2; }");
test_value_expectation!(binary_exp_019, Block, RtValue::Num(22.0), "{ 20 + 4 / 2; }");
test_value_expectation!(binary_exp_020, Block, RtValue::Num(1.0), "{ 5 - 2 * 2; }");
test_value_expectation!(
    binary_exp_021,
    Block,
    RtValue::Num(18.0),
    "{ ((2 + 4) * 3); }"
);
test_value_expectation!(
    binary_exp_022,
    Block,
    RtValue::Num(42.0),
    "{ ((3 + 4) * (6)); }"
);
test_value_expectation!(
    binary_exp_023,
    Block,
    RtValue::Num(25.0),
    "{ (5 * (2 + 3)); }"
);
test_value_expectation!(
    binary_exp_024,
    Block,
    RtValue::Num(21.0),
    "{ ((6 + 1) * 3); }"
);
test_value_expectation!(
    binary_exp_025,
    Block,
    RtValue::Num(13.0),
    "{ (2 + (3 * 4 - 1)); }"
);
test_value_expectation!(
    binary_exp_026,
    Block,
    RtValue::Num(15.0),
    "{ (2 + 3) * ((4) - 1); }"
);
test_value_expectation!(
    binary_exp_027,
    Block,
    RtValue::Num(90.0),
    "{ ((3 + 3) * (3 + 2) * (3)); }"
);
test_value_expectation!(binary_exp_028, Block, RtValue::Num(9.0), "{ (3 * (3)); }");
test_value_expectation!(
    binary_exp_029,
    Block,
    RtValue::Num(12.0),
    "{ (((2 * 3) * 2)); }"
);
test_value_expectation!(
    binary_exp_030,
    Block,
    RtValue::Num(48.0),
    "{ (8 * (3 + 3)); }"
);
test_value_expectation!(
    binary_exp_031,
    Block,
    RtValue::Num(18.0),
    "{ (((2 + 3) * 4) - 2); }"
);
test_value_expectation!(
    binary_exp_032,
    Block,
    RtValue::Num(56.0),
    "{ (((7) * 4) + 28); }"
);
test_value_expectation!(
    binary_exp_033,
    Block,
    RtValue::Num(10.0),
    "{ (5 + (5 + (5 - (5)))); }"
);
test_value_expectation!(
    binary_exp_034,
    Block,
    RtValue::Num(2.0),
    "{ ((((12 / 2) / 3))); }"
);
test_value_expectation!(
    binary_exp_035,
    Block,
    RtValue::Num(100.0),
    "{ (10 * (5 + 5)); }"
);
test_value_expectation!(
    binary_exp_036,
    Block,
    RtValue::Num(81.0),
    "{ ((9 * (4 + 5)) - (0)); }"
);
test_value_expectation!(
    binary_exp_037,
    Block,
    RtValue::Num(50.0),
    "{ (((25 + 25))); }"
);
test_value_expectation!(
    binary_exp_038,
    Block,
    RtValue::Num(64.0),
    "{ ((((8) * (4 + 4)))); }"
);
test_value_expectation!(
    binary_exp_039,
    Block,
    RtValue::Num(72.0),
    "{ (((6 * 6) + 36)); }"
);
test_value_expectation!(
    binary_exp_040,
    Block,
    RtValue::Num(3.0),
    "{ ((((25 / 5) - 2))); }"
);
