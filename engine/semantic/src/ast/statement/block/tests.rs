use crate::*;

test_success!(
    success_block_000,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            if a > 10 {
                return false;
            }
            true;
        }
        task task_b() {
            let a: bool = true;
            a = :comp:task_a(10);
            a;
        }
    };
    "#
);

test_fail!(
    fail_block_000,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            if a > 10 {
                // returning of num makes type of task Indeterminate
                return 5;
            }
            true;
        }
        task task_b() {
            let a: bool = true;
            a = :comp:task_a(10);
            a;
        }
    };
    "#
);
