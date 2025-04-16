use std::ops::RangeInclusive;

use crate::*;

test_value_expectation!(
    range_000,
    Block,
    RtValue::Range(RangeInclusive::new(10, 20)),
    "{ let a = 10..20; a; }"
);


test_value_expectation!(
    range_001,
    Block,
    RtValue::Num(55.0),
    r#"{
        let sum = 0;
        for(el, n) in 0..10 {
            sum = sum + n;
        };
        sum;
    }"#
);
