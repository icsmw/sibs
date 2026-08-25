use runtime::RtValue;
use semantic::{Script, ScriptOptions};

use crate::{ExecutionOptions, Executor, ExecutorError};

fn script() -> Script {
    Script::from_text(
        r#"
        component my_component() {
            task task_a() {
                true;
            }
        };
        "#,
        ScriptOptions::strict(),
    )
    .expect("script is prepared")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn runs_script_once() {
    let value = Executor::new(
        script(),
        ExecutionOptions::new("my_component", "task_a", std::env::temp_dir()),
    )
    .run()
    .await
    .expect("script is executed");

    assert_eq!(value, RtValue::Bool(true));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn returns_execution_failure_with_report() {
    let err = Executor::new(
        script(),
        ExecutionOptions::new("my_component", "missing_task", std::env::temp_dir()),
    )
    .run()
    .await
    .expect_err("script execution fails");

    let ExecutorError::Execution(failure) = err else {
        panic!("expected execution failure");
    };
    assert!(!failure.to_string().is_empty());
    assert!(!failure.inner().e.to_string().is_empty());
    assert!(!failure.report().expect("execution report").is_empty());
}
