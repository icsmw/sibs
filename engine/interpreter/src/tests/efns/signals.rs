use crate::*;

test_task_results!(
    signals_000,
    "my_component",
    "task_a",
    RtValue::Num(2.0),
    r#"
    component my_component() {
        task task_a() {
            join(
                `../target/debug/exit 0 400 60 60`.success(),
                :my_component:task_b(),
                :my_component:task_c(),
                :my_component:task_d(),
            );
            signals::waiters("SignalA");
        }
        task task_b() {
            `../target/debug/exit 0 500 60 60`.success();
            signals::emit("SignalA");
        }
        task task_c() {
            signals::wait("SignalA");
            `../target/debug/exit 0 200 60 60`.success();
        }
        task task_d() {
            signals::wait("SignalA");
            `../target/debug/exit 0 300 60 60`.success();
        }
    };
    "#
);
