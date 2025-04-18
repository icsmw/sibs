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
