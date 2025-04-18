use crate::*;

test_task_results!(
    skip_000,
    "comp",
    "task_a",
    RtValue::Bool(true),
    r#"
    component comp() {
        #[skip(debugging::out(true))];
        task task_a() {
            true;
        }
    };
    "#
);

test_task_results!(
    skip_001,
    "comp",
    "task_a",
    RtValue::Skipped,
    r#"
    component comp() {
        #[skip(debugging::out(false))];
        task task_a() {
            true;
        }
    };
    "#
);

test_task_results!(
    skip_002,
    "comp",
    "task_a",
    RtValue::Skipped,
    r#"
    component comp() {
        task task_a() {
            :comp:task_b();
        }
        #[skip(debugging::out(false))];
        task task_b() {
            true;
        }
    };
    "#
);

test_task_results!(
    skip_003,
    "comp",
    "task_a",
    RtValue::Num(5.0),
    r#"
    component comp() {
        #[skip(debugging::out(true))];
        task task_a() {
            :comp:task_b(5);
        }
        #[skip(v = 6, debugging::out(false))];
        task task_b(v: num) {
            v;
        }
    };
    "#
);

test_task_results!(
    skip_004,
    "comp",
    "task_a",
    RtValue::Skipped,
    r#"
    component comp() {
        task task_a() {
            :comp:task_b(5);
        }
        #[skip(v = 5, debugging::out(false))];
        task task_b(v: num) {
            v;
        }
    };
    "#
);

test_task_results!(
    skip_005,
    "comp",
    "task_a",
    RtValue::Skipped,
    r#"
    component comp() {
        task task_a() {
            :comp:task_b(5);
        }
        #[skip(hash::inspect(["../target"], [""], false))];
        task task_b(v: num) {
            v;
        }
    };
    "#
);
