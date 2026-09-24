use crate::*;
use tokio::time::{timeout, Duration};

fn panic_while_polling(_: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    Box::pin(async {
        tokio::task::yield_now().await;
        panic!("nested execution panic");
    })
}

fn panic_while_creating_future(_: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    std::panic::panic_any(String::from("nested execution panic"));
}

fn panic_with_non_string_payload(_: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
    Box::pin(async { std::panic::panic_any(42_u32) })
}

fn prepare(
    source: &str,
    target: NodeTarget<'_>,
    exec: ExecutorEmbeddedFn,
) -> (LinkedNode, SemanticCx) {
    let mut lx = lexer::Lexer::new(source, 0);
    let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, source, false);
    let node = LinkedNode::try_read(&parser, target).unwrap().unwrap();
    let mut scx = SemanticCx::new(false);
    scx.fns
        .efns
        .add(
            "test_crash",
            EmbeddedFnEntity {
                uuid: Uuid::new_v4(),
                fullname: "test_crash".into(),
                name: "test_crash".into(),
                docs: String::new(),
                args: vec![],
                result: DeterminedTy::Any,
                exec,
            },
        )
        .unwrap();
    node.initialize(&mut scx).unwrap();
    node.infer_type(&mut scx).unwrap();
    node.finalize(&mut scx).unwrap();
    (node, scx)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inner_interpret_clears_parent_value_after_panic_or_error() {
    async fn leave_parent_value(env: FnEnv) {
        env.cx
            .values()
            .set_parent_vl(ParentValue {
                value: RtValue::Num(42.0),
                link: SrcLink::default(),
            })
            .await
            .unwrap();
    }
    fn crash(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async move {
            leave_parent_value(env).await;
            panic!("panic with a pending parent value");
        })
    }
    fn fail(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async move {
            leave_parent_value(env).await;
            Err(LinkedErr::unlinked(E::SpawnFailed("injected error".into())))
        })
    }
    for exec in [crash as ExecutorEmbeddedFn, fail] {
        let (node, scx) = prepare(
            "test_crash()",
            NodeTarget::Expression(&[ExpressionId::FunctionCall]),
            exec,
        );
        let rt = runtime(RtParameters::default_from_cwd().unwrap(), scx).unwrap();
        let env = rt
            .create_interpreter_env("parent value cleanup", None)
            .await
            .unwrap();
        let cx = env.cx.clone();
        let result = node.interpret_owned(env).await;
        assert!(
            matches!(
                result,
                Err(LinkedErr {
                    e: E::ExecutionPanicked(_) | E::SpawnFailed(_),
                    ..
                })
            ),
            "{result:?}"
        );
        assert!(cx.values().withdraw_parent_vl().await.unwrap().is_none());
        timeout(Duration::from_secs(2), rt.destroy())
            .await
            .unwrap()
            .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn nested_panics_finish_jobs_and_restore_context() {
    let source = r#"{
        let value = 2;
        fn call() {
            let cb = |n: num| {
                loop { test_crash(); break; };
                n;
            };
            cb(5);
        }
        call();
    }"#;
    for (exec, expected) in [
        (
            panic_while_polling as ExecutorEmbeddedFn,
            "nested execution panic",
        ),
        (panic_while_creating_future, "nested execution panic"),
        (panic_with_non_string_payload, "non-string panic payload"),
    ] {
        let (node, scx) = prepare(source, NodeTarget::Statement(&[StatementId::Block]), exec);
        let rt = runtime(RtParameters::default_from_cwd().unwrap(), scx).unwrap();
        let env = rt
            .create_interpreter_env("panic cleanup", None)
            .await
            .unwrap();
        let cx = env.cx.clone();
        cx.scopes().enter(&Uuid::new_v4()).await.unwrap();
        cx.values()
            .insert("value", RtValue::Num(1.0))
            .await
            .unwrap();

        let result = timeout(Duration::from_secs(2), node.interpret_owned(env))
            .await
            .unwrap();
        assert!(
            matches!(&result, Err(LinkedErr { e: E::ExecutionPanicked(message), .. })
            if message == expected),
            "{result:?}"
        );
        assert_eq!(
            cx.values().lookup("value").await.unwrap().as_deref(),
            Some(&RtValue::Num(1.0))
        );
        assert!(matches!(
            cx.loops().close().await,
            Err(E::NoOpenLoopsToClose)
        ));
        assert!(matches!(
            cx.returns().close_cx().await,
            Err(E::NoOpenReturnCXsToClose)
        ));
        cx.scopes().leave().await.unwrap();
        assert!(matches!(
            cx.scopes().leave().await,
            Err(E::AttemptToLeaveRootContextLevel)
        ));
        timeout(Duration::from_secs(2), rt.destroy())
            .await
            .unwrap()
            .unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn join_recovers_from_a_panic_in_a_nested_task() {
    let source = r#"
        component comp() {
            task run() { join(:comp:crash(), :comp:ok()); }
            task crash() { test_crash(); }
            task ok() { 42; }
        };
    "#;
    let (node, scx) = prepare(
        source,
        NodeTarget::Root(&[RootId::Anchor]),
        panic_while_polling,
    );
    let rt = runtime(
        RtParameters::new("comp", "run", vec![], std::env::current_dir().unwrap()),
        scx,
    )
    .unwrap();
    let env = rt
        .create_interpreter_env("nested task panic", None)
        .await
        .unwrap();
    let result = timeout(Duration::from_secs(2), node.interpret_owned(env))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        result,
        RtValue::Vec(vec![
            RtValue::Error(E::ExecutionPanicked("nested execution panic".into()).to_string()),
            RtValue::Num(42.0),
        ])
    );
    timeout(Duration::from_secs(2), rt.destroy())
        .await
        .unwrap()
        .unwrap();
}
