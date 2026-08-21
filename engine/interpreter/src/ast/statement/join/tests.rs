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
async fn test_fail_join_004() {
    use tokio::time::{timeout, Duration};

    let content = r#"
    component my_component() {
        task task_a() {
            join(
                :my_component:task_b(),
                :my_component:task_c(),
            );
            true;
        }
        task task_b() {
            `same very fake command`.success();
        }
        task task_c() {
            signals::wait("NeverEmitted");
            true;
        }
    };
    "#;

    let mut lx = lexer::Lexer::new(&content, 0);
    let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, content, false);
    let node = Anchor::read(&mut parser)
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
    let cx = rt
        .create_cx(Uuid::new_v4(), "Test", None)
        .await
        .expect("ExecutionContext created");
    let result = timeout(
        Duration::from_secs(5),
        node.interpret(rt.clone(), cx.clone()),
    )
    .await
    .expect("Join finished without hanging");
    let _ = rt.destroy().await;

    assert!(result.is_err());
}
