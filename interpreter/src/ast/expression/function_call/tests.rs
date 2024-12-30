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
