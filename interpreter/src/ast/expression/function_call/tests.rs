use crate::*;

test_task_results!(
    function_call_000,
    "my_component",
    "task_a",
    RtValue::Num(10.0),
    r#"
    mod aaa {
        fn sum(a: num, b: num) {
           a + b;
        };
    };
    component my_component() {
        task task_a() {
            let a = aaa::sum(5, 5);
            a;
        }
    };
    "#
);

test_value_expectation!(
    embedded_function_call_000,
    Block,
    RtValue::Num(10.0),
    "{ math::sum(5, 5); }"
);

test_value_expectation!(
    embedded_function_call_001,
    Block,
    RtValue::Void,
    r#"{ console::print("Hello"); }"#
);

test_value_expectation!(
    closure_function_call_000,
    Block,
    RtValue::Num(10.0),
    r#"{
        let cb = |n: num| { n * 2;};
        let a = cb(5);
        a;
    }"#
);

test_value_expectation!(
    closure_function_call_001,
    Block,
    RtValue::Num(15.0),
    r#"{
        fn sum(a: num, b: num) {
            a + b;
        }
        let cb = |n: num| { n * 2;};
        let a = sum(5, cb(5));
        a;
    }"#
);

test_value_expectation!(
    closure_function_call_002,
    Block,
    RtValue::Num(15.0),
    r#"{
        fn sum(a: num, b: num, cb: |n: num|: num) {
            a + b + cb(5);
        }
        let cb = |n: num| { n * 2;};
        let a = sum(5, 5, cb);
        a;
    }"#
);
