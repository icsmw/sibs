use crate::*;

// test_value_expectation!(
//     function_call_000,
//     Anchor,
//     RtValue::Num(10.0),
//     r#"
//     mod aaa {
//         fn sum(a: num, b: num) {
//            a + b;
//         };
//     };
//     component my_component() {
//         task task_a() {
//             let a = aaa::sum(5, 5);
//             a;
//         }
//     };
//     "#
// );

#[tokio::test]
async fn run_task() {
    use crate::*;
    let expectation = RtValue::Num(10.0);
    let component_name = "my_component";
    let task_name = "task_a";
    let content = r#"
    mod aaa {
        fn sum(a: num, b: num) {
           a + b;
        };
    };
    component my_component() {
        task task_a() {
            let a = aaa::sum(5, 5);
            a;
        }
    };
    "#;
    let mut lx = lexer::Lexer::new(content, 0);
    let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, content);
    let node = Anchor::read(&mut parser);
    if let Err(err) = &node {
        eprintln!("{}", parser.report_err(err).expect("Reporting error"));
    }
    let node = node
        .expect("Node is parsed without errors")
        .expect("Node is parsed");
    let mut scx = SemanticCx::default();
    let result = node.initialize(&mut scx);
    if let Err(err) = &result {
        eprintln!("{}", parser.report_err(err).expect("Reporting error"));
    }
    assert!(result.is_ok());
    let result = node.infer_type(&mut scx);
    if let Err(err) = &result {
        eprintln!("{}", parser.report_err(err).expect("Reporting error"));
    }
    assert!(result.is_ok());
    let result = node.finalize(&mut scx);
    if let Err(err) = &result {
        eprintln!("{}", parser.report_err(err).expect("Reporting error"));
    }
    assert!(result.is_ok());
    let rt = Runtime::new(
        RtParameters::new(component_name, task_name, Vec::new(), PathBuf::new()),
        scx.table,
        scx.fns,
    );
    let vl = node.interpret(rt.clone()).await;
    if let Err(err) = &vl {
        eprintln!("{err:?}");
        eprintln!("{}", parser.report_err(err).expect("Reporting error"));
    }
    let _ = rt.destroy().await;
    assert!(vl.is_ok());
    let vl = vl.unwrap();
    assert!(
        vl == expectation,
        "Values are not equal: {:?} vs {:?}",
        vl,
        expectation
    );
}
