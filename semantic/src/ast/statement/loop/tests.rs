use crate::*;

test_success!(
    success_loop_000,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                if a > 10 {
                    break;
                }
            }
        }
    };
    "#
);

test_success!(
    success_loop_001,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                if a > 10 {
                    break;
                }
                let b = 0;
                loop {
                    b += 1;
                    if b > 10 {
                        break;
                    }
                }
            }
        }
    };
    "#
);

test_success!(
    success_loop_002,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                if a > 10 {
                    return;
                }
            }
        }
    };
    "#
);

test_success!(
    success_loop_003,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                if a > 10 {
                    return true;
                }
                let b = 0;
                loop {
                    b += 1;
                    if b > 10 {
                        return false;
                    }
                }
            }
        }
    };
    "#
);

test_fail!(
    fail_loop_000,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
            }
        }
    };
    "#
);

test_fail!(
    fail_loop_001,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                if a > 10 {
                    break;
                }
                let b = 0;
                loop {
                    b += 1;
                }
            }
        }
    };
    "#
);

test_fail!(
    fail_loop_002,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                a += 1;
                let b = 0;
                loop {
                    b += 1;
                    if b > 10 {
                        break;
                    }
                }
            }
        }
    };
    "#
);

test_fail!(
    fail_loop_003,
    Anchor,
    r#"
    component comp() { 
        task task_a(a: num) {
            loop {
                fn test(a: str, b: num, c: bool, cb: |n: num|: num) {
                    a;
                }
                a += 1;
                if a > 10 {
                    return;
                }                
            }
        }
    };
    "#
);
