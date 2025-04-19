use crate::*;

test_success!(
    task_000,
    Anchor,
    r#"
    component comp() { 
        task task_a() {
            true;
        }
        task task_b(a: num, b: str) {
            true;
        }
        task task_c(a: bool, b: Vec<str>) {
            true;
        }
    };
    "#
);

test_fail!(
    task_000,
    Anchor,
    r#"
    component comp() { 
        task task_a() {
            true;
        }
        task task_b(a: num, b: str, cb: |n: num|: num) {
            true;
        }
    };
    "#
);
