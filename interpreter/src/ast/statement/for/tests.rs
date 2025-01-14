use crate::*;

test_value_expectation!(
    r#for_000,
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
