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

test_value_expectation!(
    r#while_001,
    Block,
    RtValue::Num(10.0),
    r#"{
        let n = 0;
        while (n < 20) {
            n += 1;
            if n == 10 {
                break;
            };
        };
        n;
    }"#
);