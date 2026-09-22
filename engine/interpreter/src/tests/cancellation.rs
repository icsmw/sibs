use crate::*;
use tokio::time::{timeout, Duration};

async fn cancel_execution(body: &str, waiters: usize) {
    let content =
        format!("component comp() {{ task run() {{ {body}; signals::emit(\"After\"); }} task wait() {{ signals::wait(\"Never\"); }} }};");
    let mut lx = lexer::Lexer::new(&content, 0);
    let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &content, false);
    let node = Anchor::read(&parser).unwrap().unwrap();
    let mut scx = SemanticCx::new(false);
    functions::register(&mut scx.fns.efns).unwrap();
    node.initialize(&mut scx).unwrap();
    node.infer_type(&mut scx).unwrap();
    node.finalize(&mut scx).unwrap();
    let rt = runtime(
        RtParameters::new("comp", "run", Vec::new(), std::env::current_dir().unwrap()),
        scx,
    )
    .unwrap();
    let env = rt
        .create_interpreter_env("cancellation test", None)
        .await
        .unwrap();
    let job = env.job.clone();
    job.start().started::<String>(None).await.unwrap();
    let ready = rt.signals().wait_signal("Ready").await.unwrap().unwrap();
    let after = rt.signals().wait_signal("After").await.unwrap().unwrap();
    let execution = tokio::spawn(async move { node.interpret(env).await });
    timeout(Duration::from_secs(5), async {
        if waiters == 0 {
            ready.cancelled().await;
        } else {
            while rt.signals().waiters_signal("Never").await.unwrap() < waiters {
                tokio::task::yield_now().await;
            }
        }
    })
    .await
    .expect("execution reached the cancellation point");
    // The caller owns the root transition; executors own their descendant jobs.
    job.cancel().cancelling().await.unwrap();
    let result = timeout(Duration::from_secs(5), execution)
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(
            result,
            Err(LinkedErr {
                e: E::Cancelled,
                ..
            })
        ),
        "{result:?}"
    );
    assert!(
        !after.is_cancelled(),
        "execution continued after cancellation"
    );
    // Terminal updates validate the entire subtree, not only direct children.
    job.cancel()
        .cancelled::<String>(None)
        .await
        .expect("all descendants finished");
    rt.destroy().await.unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_signal_wait() {
    cancel_execution("signals::wait(\"Never\")", 1).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_join_drains_both_branches() {
    cancel_execution("join(:comp:wait(), :comp:wait())", 2).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_loop() {
    cancel_execution(
        "let n = 0; loop { if n == 0 { signals::emit(\"Ready\"); n = 1; }; if n < 0 { break; }; }",
        0,
    )
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_while() {
    cancel_execution(
        "let n = 0; while n < 2 { if n == 0 { signals::emit(\"Ready\"); n = 1; }; }",
        0,
    )
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancel_for_body() {
    cancel_execution("for n in [1, 2] { signals::wait(\"Never\"); }", 1).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancelled_block_restores_scope_and_closes_loop() {
    let content = r#"{
        let value = 2;
        loop {
            signals::wait("Never");
            break;
        };
    }"#;
    let mut lx = lexer::Lexer::new(content, 0);
    let parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, content, false);
    let node = Block::read(&parser).unwrap().unwrap();
    let mut scx = SemanticCx::new(false);
    functions::register(&mut scx.fns.efns).unwrap();
    node.initialize(&mut scx).unwrap();
    node.infer_type(&mut scx).unwrap();
    node.finalize(&mut scx).unwrap();
    let rt = runtime(RtParameters::default_from_cwd().unwrap(), scx).unwrap();
    let env = rt
        .create_interpreter_env("block cleanup", None)
        .await
        .unwrap();
    let cx = env.cx.clone();
    let job = env.job.clone();
    cx.scopes().enter(&Uuid::new_v4()).await.unwrap();
    cx.values()
        .insert("value", RtValue::Num(1.0))
        .await
        .unwrap();
    let execution = tokio::spawn(async move { node.interpret(env).await });
    timeout(Duration::from_secs(5), async {
        while rt.signals().waiters_signal("Never").await.unwrap() == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    job.cancel().cancelling().await.unwrap();
    let result = timeout(Duration::from_secs(5), execution)
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(
        result,
        Err(LinkedErr {
            e: E::Cancelled,
            ..
        })
    ));
    assert_eq!(
        cx.values().lookup("value").await.unwrap().as_deref(),
        Some(&RtValue::Num(1.0))
    );
    assert!(matches!(
        cx.loops().close().await,
        Err(E::NoOpenLoopsToClose)
    ));
    cx.scopes().leave().await.unwrap();
    assert!(matches!(
        cx.scopes().leave().await,
        Err(E::AttemptToLeaveRootContextLevel)
    ));
    job.cancel().cancelled::<String>(None).await.unwrap();
    cx.close().await.unwrap();
    rt.destroy().await.unwrap();
}
