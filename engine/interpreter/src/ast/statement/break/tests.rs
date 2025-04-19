use crate::*;

test_value_expectation!(
    r#break_000,
    Block,
    RtValue::Num(15.0),
    r#"{
        let sum = 0;
        for(el, n) in 0..10 {
            sum = sum + n;
            if sum == 15 {
                break;
            };
        };
        sum;
    }"#
);

test_value_expectation!(
    r#break_001,
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

test_value_expectation!(
    r#break_002,
    Block,
    RtValue::Num(20.0),
    r#"{
        let n = 0;
        let a = 0;
        while (n < 20) {
            n += 1;
            while (a < 20) {
                if a == 10 {
                    break;
                };
                a += 1;
            };
            if n == 10 {
                break;
            };
        };
        n + a;
    }"#
);

test_fail!(
    r#break_000,
    Block,
    r#"{
        // Attempt to break without loop
        break;
    }"#
);

test_fail!(
    r#break_001,
    Block,
    r#"{
        let sum = 15;
        if sum == 15 {
            // Attempt to break without loop
            break;
        };
        sum;
    }"#
);

test_fail!(
    r#break_002,
    Block,
    r#"{
        let sum = 0;
        for(el, n) in 0..10 {
            sum = sum + n;
            if sum == 15 {
                break;
            };
        };
        if sum == 15 {
            // Attempt to break without loop
            break;
        };
        sum;
    }"#
);
