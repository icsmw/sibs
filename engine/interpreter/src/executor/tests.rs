use runtime::RtValue;

use crate::{ExecutionOptions, Executor, ExecutorError, InterContext, Script, ScriptOptions};

const CONTENT: &str = "component comp() { task run() { true; } };";

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn runs_script_once() {
    let mut ctx = InterContext::default();
    Script::from_text(CONTENT, ScriptOptions::strict(), &mut ctx).unwrap();
    let options = ExecutionOptions::new("comp", "run", std::env::temp_dir());
    let value = Executor::new(options.clone()).run(&mut ctx).await.unwrap();

    assert_eq!(value, RtValue::Bool(true));
    assert!(ctx.get_semantic_cx().is_none());
    assert!(ctx.get_anchor().is_some());
    assert!(ctx.get_diagnostics().is_some());
    let second = Executor::new(options).run(&mut ctx).await;
    assert!(
        matches!(second, Err(ExecutorError::InvalidContext)),
        "{second:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn execution_failure_can_be_reported_from_context() {
    let mut ctx = InterContext::default();
    Script::from_text(CONTENT, ScriptOptions::strict(), &mut ctx).unwrap();
    let err = Executor::new(ExecutionOptions::new(
        "comp",
        "missing_task",
        std::env::temp_dir(),
    ))
    .run(&mut ctx)
    .await
    .unwrap_err();

    let ExecutorError::Execution(err) = err else {
        panic!("expected execution failure, got {err:?}");
    };
    let mut output = Vec::new();
    ctx.get_diagnostics()
        .unwrap()
        .err(&err, &mut output)
        .unwrap();
    let report = String::from_utf8(output).unwrap();
    assert!(report.contains("missing_task"), "{report}");
    assert!(report.contains(CONTENT), "{report}");
    assert!(report.contains('^'), "{report}");
}
