use crate::*;

test_success!(
    range_000,
    Block,
    r#"{
        let sum: num = 0;
        for (el, n) in 0..10 {
            sum += el;
        };
    }"#
);
