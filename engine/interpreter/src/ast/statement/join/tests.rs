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

test_task_results!(
    join_002,
    "my_component",
    "task_a",
    RtValue::Vec(vec![RtValue::Num(1.0), RtValue::Num(2.0)]),
    r#"
    component my_component() {
        task task_a() {
            join(
                :my_component:worker(1),
                :my_component:worker(2),
            );
        }
        task worker(v: num) {
            v;
        }
    };
    "#
);

test_task_results!(
    join_003,
    "my_component",
    "task_a",
    RtValue::Vec(vec![RtValue::Num(1.0), RtValue::Num(2.0)]),
    r#"
    component my_component() {
        task task_a() {
            join(
                :my_component:worker(1),
                :my_component:worker(2),
            );
        }
        task worker(v: num) {
            join(
                :my_component:leaf(v),
                :my_component:leaf(v + 10),
            );
            v;
        }
        task leaf(v: num) {
            v;
        }
    };
    "#
);

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn join_branch_error_does_not_cancel_a_waiting_branch() {
    use tokio::time::{timeout, Duration};

    let content = r#"
    component my_component() {
        task task_a() {
            join(
                :my_component:task_c(),
                :my_component:task_b(),
            );
        }
        task task_b() {
            while signals::waiters("NeverEmitted") < 1 {};
            `same very fake command`.success();
        }
        task task_c() {
            signals::wait("NeverEmitted");
            true;
        }
    };
    "#;

    let mut lx = lexer::Lexer::new(content, 0);
    let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, content, false);
    let node = Anchor::read(&parser)
        .expect("Node is parsed without errors")
        .expect("Node is parsed");
    let mut scx = SemanticCx::new(false);
    functions::register(&mut scx.fns.efns).expect("functions are registred");
    assert!(node.initialize(&mut scx).is_ok());
    assert!(node.infer_type(&mut scx).is_ok());
    assert!(node.finalize(&mut scx).is_ok());

    let params = RtParameters::new(
        "my_component",
        "task_a",
        Vec::new(),
        std::env::current_dir().expect("Current folder detected"),
    );
    let rt = runtime(params, scx).expect("Runtime created");
    let env = rt
        .create_interpreter_env("Test", None)
        .await
        .expect("InterpreterEnvironment created");
    let job = env.job.clone();
    job.start().started::<String>(None).await.unwrap();
    let execution = node.interpret(env);
    tokio::pin!(execution);
    timeout(Duration::from_secs(5), async {
        tokio::select! {
            result = &mut execution => panic!("join returned before the waiting branch was released: {result:?}"),
            _ = async {
                while rt.signals().waiters_signal("NeverEmitted").await.unwrap() == 0 {
                    tokio::task::yield_now().await;
                }
            } => {}
        }
    }).await.unwrap();
    rt.signals().emit_signal("NeverEmitted").await.unwrap();
    let result = timeout(Duration::from_secs(5), execution).await.unwrap();
    assert!(
        matches!(&result, Ok(RtValue::Vec(values)) if matches!(values.as_slice(),
            [RtValue::Bool(true), RtValue::Error(message)] if !message.is_empty())),
        "{result:?}"
    );
    job.done()
        .success::<String>(None)
        .await
        .expect("join drained every branch");
    rt.destroy().await.unwrap();
}

// Construct branches at the AST boundary so these unit tests exercise the real
// Join interpreter without requiring an external executable or changing its parser.
fn join_with_executors(branches: &[(&str, ExecutorEmbeddedFn)]) -> (Join, SemanticCx) {
    let mut lx = lexer::Lexer::new("join()", 0);
    let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, "join()", false);
    let mut join = Join::read(&parser).unwrap().unwrap();
    let mut scx = SemanticCx::new(false);
    for &(name, exec) in branches {
        scx.fns
            .efns
            .add(
                name,
                EmbeddedFnEntity {
                    uuid: Uuid::new_v4(),
                    fullname: name.into(),
                    name: name.into(),
                    docs: String::new(),
                    args: vec![],
                    result: DeterminedTy::Any,
                    exec,
                },
            )
            .unwrap();
        let source = format!("{name}()");
        let mut lx = lexer::Lexer::new(&source, 0);
        let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &source, false);
        let call = FunctionCall::read(&parser).unwrap().unwrap();
        let node = LinkedNode::from_node(Node::Expression(Expression::FunctionCall(call)));
        node.initialize(&mut scx).unwrap();
        node.infer_type(&mut scx).unwrap();
        node.finalize(&mut scx).unwrap();
        join.commands.push(node);
    }
    (join, scx)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn join_returns_success_and_failed_command_statuses_in_source_order() {
    use tokio::time::{timeout, Duration};

    fn slow(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async move {
            // This branch cannot complete before the failing branch reaches its result.
            if let Some(signal) = env.rt.signals().wait_signal("FailureReady").await.unwrap() {
                signal.cancelled().await;
            }
            Ok(RtValue::ExecuteResult(ExecuteResult::Success(vec![
                RtValue::Str("slow".into()),
            ])))
        })
    }
    fn fast(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async move {
            env.rt.signals().emit_signal("FailureReady").await.unwrap();
            Ok(RtValue::ExecuteResult(ExecuteResult::Failed(
                Some(7),
                vec![RtValue::Str("fast".into())],
            )))
        })
    }
    let (node, scx) = join_with_executors(&[("test_slow", slow), ("test_fast", fast)]);
    let rt = runtime(RtParameters::default_from_cwd().unwrap(), scx).unwrap();
    let env = rt
        .create_interpreter_env("join command results", None)
        .await
        .unwrap();
    let job = env.job.clone();
    job.start().started::<String>(None).await.unwrap();
    let result = timeout(Duration::from_secs(5), node.interpret(env))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        result,
        RtValue::Vec(vec![
            RtValue::ExecuteResult(ExecuteResult::Success(vec![RtValue::Str("slow".into())])),
            RtValue::ExecuteResult(ExecuteResult::Failed(
                Some(7),
                vec![RtValue::Str("fast".into())]
            )),
        ])
    );
    assert!(!job.cancel().is_cancelled());
    job.done()
        .success::<String>(None)
        .await
        .expect("all branch jobs finished");
    rt.destroy().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn interpret_collects_branch_and_handle_errors_after_all_branches_finish() {
    use tokio::time::{timeout, Duration};

    fn gate(env: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async move {
            let release = env
                .rt
                .signals()
                .wait_signal("Release")
                .await
                .unwrap()
                .unwrap();
            env.rt.signals().emit_signal("Ready").await.unwrap();
            release.cancelled().await;
            Ok(RtValue::Num(42.0))
        })
    }
    fn crash(_: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async { panic!("injected branch panic") })
    }
    fn fail(_: FnEnv) -> RtPinnedResult<'static, LinkedErr<E>> {
        Box::pin(async {
            Err(LinkedErr::unlinked(E::SpawnFailed(
                "injected branch error".into(),
            )))
        })
    }
    let (node, scx) = join_with_executors(&[
        ("test_gate", gate),
        ("test_crash", crash),
        ("test_fail", fail),
    ]);
    let rt = runtime(RtParameters::default_from_cwd().unwrap(), scx).unwrap();
    let env = rt
        .create_interpreter_env("join outcomes", None)
        .await
        .unwrap();
    let ready = rt.signals().wait_signal("Ready").await.unwrap().unwrap();
    // Call Join directly: a deliberately panicking node cannot finalize its own
    // job. This tests aggregation, not recovery of that broken lifecycle.
    let execution = node.interpret(env);
    tokio::pin!(execution);
    timeout(Duration::from_secs(5), async {
        tokio::select! {
            result = &mut execution => panic!("join returned before release: {result:?}"),
            _ = ready.cancelled() => {}
        }
    })
    .await
    .unwrap();
    assert!(futures::poll!(&mut execution).is_pending());
    rt.signals().emit_signal("Release").await.unwrap();
    let result = timeout(Duration::from_secs(5), execution)
        .await
        .unwrap()
        .unwrap();
    let RtValue::Vec(values) = result else {
        panic!("expected result vector")
    };
    assert!(
        matches!(values.as_slice(), [RtValue::Num(42.0), RtValue::Error(panic), RtValue::Error(error)]
        if panic.contains("injected branch panic") && error == &E::SpawnFailed("injected branch error".into()).to_string()),
        "{values:?}"
    );
    rt.destroy().await.unwrap();
}
