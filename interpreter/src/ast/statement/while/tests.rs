use crate::*;

test_value_expectation!(
    r#while_000,
    Block,
    RtValue::Num(10.0),
    r#"{
        let n = 0;
        while (n < 10) {
            n += 1;
        };
        n;
    }"#
);
