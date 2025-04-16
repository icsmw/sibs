mod efns;

#[macro_export]
macro_rules! test_value_expectation {
    ($fn_name:ident, $element_ref:expr, $expectation:expr, $content:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content);
                let node = $element_ref::read(&mut parser);
                if let Err(err) = &node {
                    eprintln!("{}", parser.report_err(err).expect("Reporting error"));
                }
                let node = node.expect("Node is parsed without errors")
                    .expect("Node is parsed");
                let mut scx = SemanticCx::default();
                functions::register(&mut scx.fns.efns).expect("functions are registred");
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
                    RtParameters::default_from_cwd().expect("RtParameter created"),
                    scx.table,
                    into_rt_ufns(scx.fns),
                    into_rt_tasks(scx.tasks),
                ).expect("Runtime created");
                let cx = rt.create_cx(Uuid::new_v4(), "Test", None).await.expect("Context created");
                let vl = node.interpret(rt.clone(), cx.clone()).await;
                if let Err(err) = &vl {
                    eprintln!("{err:?}");
                    eprintln!("{}", parser.report_err(err).expect("Reporting error"));
                }
                let _ = rt.destroy().await;
                assert!(vl.is_ok());
                let vl = vl.unwrap();
                assert!(
                    vl == $expectation,
                    "Values are not equal: {:?} vs {:?}",
                    vl,
                    $expectation
                );
            }
        }
    };
}

#[macro_export]
macro_rules! test_fail {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_fail $fn_name >]() {
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content);
                let node = $element_ref::read(&mut parser);
                if let Err(err) = &node {
                    eprintln!("{}", parser.report_err(err).expect("Reporting error"));
                }
                let node = node.expect("Node is parsed without errors")
                    .expect("Node is parsed");
                let mut scx = SemanticCx::default();
                functions::register(&mut scx.fns.efns).expect("functions are registred");
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
                    RtParameters::default_from_cwd().expect("RtParameter created"),
                    scx.table,
                    into_rt_ufns(scx.fns),
                    into_rt_tasks(scx.tasks),
                ).expect("Runtime created");
                let cx = rt.create_cx(Uuid::new_v4(), "Test", None).await.expect("Context created");
                let vl = node.interpret(rt.clone(), cx.clone()).await;
                assert!(vl.is_err());
                let _ = rt.destroy().await;
            }
        }
    };
}

#[macro_export]
macro_rules! test_task_results {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $expectation:expr, $content:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                use std::path::PathBuf;

                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, $content);
                let node = Anchor::read(&mut parser);
                if let Err(err) = &node {
                    eprintln!("{}", parser.report_err(err).expect("Reporting error"));
                }
                let node = node
                    .expect("Node is parsed without errors")
                    .expect("Node is parsed");
                let mut scx = SemanticCx::default();
                functions::register(&mut scx.fns.efns).expect("functions are registred");
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
                    RtParameters::new($component_name, $task_name, Vec::new(), PathBuf::new()),
                    scx.table,
                    into_rt_ufns(scx.fns),
                    into_rt_tasks(scx.tasks),
                ).expect("Runtime created");
                let cx = rt.create_cx(Uuid::new_v4(), "Test", None).await.expect("Context created");
                let vl = node.interpret(rt.clone(), cx.clone()).await;
                if let Err(err) = &vl {
                    eprintln!("{err:?}");
                    eprintln!("{}", parser.report_err(err).expect("Reporting error"));
                }
                let _ = rt.destroy().await;
                assert!(vl.is_ok());
                let vl = vl.unwrap();
                assert!(
                    vl == $expectation,
                    "Values are not equal: {:?} vs {:?}",
                    vl,
                    $expectation
                );
            }
        }
    };
}
