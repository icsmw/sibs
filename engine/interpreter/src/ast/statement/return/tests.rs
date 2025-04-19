use crate::*;

test_task_results!(
    return_000,
    "comp",
    "task_b",
    RtValue::Num(100.0),
    r#"
    component comp() {
        task task_a(a: num) {
            if a > 10 {
                if a > 10 {
                    return 100;
                };
            };
            a + 10;
        }
        task task_b() {
            :comp:task_a(11);
        }
    };
    "#
);

test_task_results!(
    return_001,
    "my_component",
    "task_a",
    RtValue::Num(100.0),
    r#"
    mod aaa {
        fn sum(a: num, b: num) {
            if a > 10 {
                return 100;
            }
            a + b
        };
    };
    component my_component() {
        task task_a() {
            let a = aaa::sum(11, 5);
            a;
        }
    };
    "#
);

test_task_results!(
    return_002,
    "my_component",
    "task_a",
    RtValue::Num(200.0),
    r#"
    mod aaa {
        fn sum(a: num, b: num) {
            if a > 10 {
                return 100;
            }
            a + b
        };
    };
    component my_component() {
        task task_a() {
            let a = aaa::sum(11, 5);
            if a == 100 {
                return 200;
            }
            a;
        }
    };
    "#
);
