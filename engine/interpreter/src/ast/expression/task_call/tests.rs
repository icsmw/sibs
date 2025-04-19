use crate::*;

test_task_results!(
    task_call_000,
    "comp",
    "task_b",
    RtValue::Num(10.0),
    r#"
    component comp() {
        task task_a() {
            10;
        }
        task task_b() {
            :comp:task_a();
        }
    };
    "#
);

test_task_results!(
    task_call_001,
    "comp",
    "task_b",
    RtValue::Num(10.0),
    r#"
    component comp() {
        task task_a(a: num) {
            a;
        }
        task task_b() {
            :comp:task_a(10);
        }
    };
    "#
);
