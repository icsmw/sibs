use crate::*;

test_value_expectation!(
    loop_000,
    Block,
    RtValue::Num(10.0),
    r#"{
        let sum = 0;
        loop {
            sum += 1;
            if sum >= 10 {
                break;
            }
        };
        sum;
    }"#
);

test_task_results!(
    loop_001,
    "comp",
    "task_b",
    RtValue::Num(0.0),
    r#"
    component comp() {
        task task_a(a: num) {
            loop {
                a += 100;
                if a > 1000 {
                    return 0;
                }
            }
            a;
        }
        task task_b() {
            :comp:task_a(10);
        }
    };
    "#
);

test_task_results!(
    loop_002,
    "my_component",
    "task_a",
    RtValue::Num(0.0),
    r#"
    mod aaa {
        fn sum(a: num) {
            loop {
                a += 100;
                if a > 1000 {
                    return 0;
                }
            }
            a;
        };
    };
    component my_component() {
        task task_a() {
            let a = aaa::sum(10);
            a;
        }
    };
    "#
);

test_fail!(
    loop_000,
    Block,
    r#"{
        let sum = 0;
        loop {
            sum += 1;
            if sum >= 10 {
                return 0;
            }
        };
        sum;
    }"#
);
