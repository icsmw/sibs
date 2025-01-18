use crate::*;

test_value_expectation!(
    compound_assignments_000,
    Block,
    RtValue::Num(55.0),
    r#"{
        let sum = 0;
        for(el, n) in 0..10 {
            sum += el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_001,
    Block,
    RtValue::Num(0.0),
    r#"{
        let sum = 55;
        for(el, n) in 0..10 {
            sum -= el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_002,
    Block,
    RtValue::Num(120.0),
    r#"{
        let sum = 1;
        for(el, n) in 0..5 {
            if el != 0 {
                sum *= el;
            }
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_004,
    Block,
    RtValue::Num(5.0),
    r#"{
        let sum = 10;
        for(el, n) in 1..2 {
            sum /= el;
        };
        sum;
    }"#
);

test_value_expectation!(
    compound_assignments_005,
    Block,
    RtValue::Num(20.0),
    r#"{
        let sum = 10;
        sum += 10;
    }"#
);

// test_value_expectation!(
//     compound_assignments_005,
//     Block,
//     RtValue::Str(String::from("HelloHelloHelloHelloHello")),
//     r#"{
//         let full = "";
//         for(el, n) in 0..4 {
//             full += "Hello";
//         };
//         full;
//     }"#
// );
