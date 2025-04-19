use crate::*;

test_task_results!(
    task_000,
    "comp",
    "task_b",
    RtValue::Num(100.0),
    r#"
    component comp() {
        task task_a(a: num) {
            if a > 10 {
                return 100;
            };
            a + 10;
        }
        task task_b() {
            :comp:task_a(11);
        }
    };
    "#
);
