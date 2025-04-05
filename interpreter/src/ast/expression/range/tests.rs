use std::ops::RangeInclusive;

use crate::*;

test_value_expectation!(
    range_000,
    Block,
    RtValue::Range(RangeInclusive::new(10, 20)),
    "{ let a = 10..20; a; }"
);
