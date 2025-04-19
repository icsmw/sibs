use crate::*;

test_value_expectation!(
    join_000,
    Block,
    RtValue::Bool(true),
    r#"
    {
        join(
            `../target/debug/exit 0 500 60 60`,
            `../target/debug/exit 0 400 60 60`,
            `../target/debug/exit 0 100 60 60`,
            `../target/debug/exit 0 200 60 60`,
            `../target/debug/exit 0 300 60 60`,
        );
        true;
    }"#
);

test_task_results!(
    join_001,
    "my_component",
    "task_a",
    RtValue::Bool(true),
    r#"
    component my_component() {
        task task_a() {
            join(
                `../target/debug/exit 0 500 60 60`.success(),
                :my_component:task_b().success(),
                :my_component:task_c().success(),
                :my_component:task_d().success(),
            );
            true;
        }
        task task_b() {
            `../target/debug/exit 0 400 60 60`;
        }
        task task_c() {
            `../target/debug/exit 0 200 60 60`;
        }
        task task_d() {
            `../target/debug/exit 0 300 60 60`;
        }
    };
    "#
);
