use crate::*;

test_value_expectation!(binary_exp, Block, RtValue::Num(10.0), "{ 5 + 5; }");
