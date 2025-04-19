use crate::*;

test_success!(
    task_call_000,
    Anchor,
    r#"
    component comp() { 
        task task_a() {
            true;
        }
        task task_b() {
            :comp:task_a();
        }
    };
    "#
);

test_success!(
    task_call_001,
    Anchor,
    r#"
    component comp() { 
        task task_a() {
            :comp:task_a();
            true;
        }
    };
    "#
);

test_fail!(
    task_call_000,
    Anchor,
    r#"
    component comp() { 
        task task_b() {
            :comp:task_a();
        }
    };
    "#
);
