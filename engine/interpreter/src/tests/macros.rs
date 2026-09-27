#[macro_export]
macro_rules! test_value_expectation {
    ($fn_name:ident, $element_ref:expr, $expectation:expr, $content:literal) => {
        paste::item! {
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content, false);
                let node = $element_ref::read_as_linked(&mut parser);
                let diagnostics: diagnostics::Diagnostics<ParserError> = parser.try_into().expect("Parser diagnostics are available");
                if let Err(err) = &node {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                let node = node.expect("Node is parsed without errors")
                    .expect("Node is parsed");
                let mut scx = SemanticCx::new(false);
                functions::register(&mut scx.fns.efns).expect("functions are registered");
                let result = node.initialize(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let result = node.infer_type(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let result = node.finalize(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let params = RtParameters::default_from_cwd().expect("RtParameter created");
                let rt = runtime(params, scx).expect("Runtime created");
                let env = rt.create_interpreter_env("Test", None).await.expect("InterpreterEnvironment created");
                let vl = node.interpret_owned(env).await;
                if let Err(err) = &vl {
                    eprintln!("{err:?}");
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                rt.destroy().await.expect("Runtime shuts down");
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
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_fail $fn_name >]() {
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content, false);
                let node = $element_ref::read_as_linked(&mut parser);
                let diagnostics: diagnostics::Diagnostics<ParserError> = parser.try_into().expect("Parser diagnostics are available");
                if let Err(err) = &node {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                let node = node.expect("Node is parsed without errors")
                    .expect("Node is parsed");
                let mut scx = SemanticCx::new(false);
                functions::register(&mut scx.fns.efns).expect("functions are registered");
                let result = node.initialize(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let result = node.infer_type(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let result = node.finalize(&mut scx);
                if let Err(err) = &result {
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                }
                assert!(result.is_ok());
                let params = RtParameters::default_from_cwd().expect("RtParameter created");
                let rt = runtime(params, scx).expect("Runtime created");
                let env = rt.create_interpreter_env("Test", None).await.expect("InterpreterEnvironment created");
                let vl = node.interpret_owned(env).await;
                assert!(vl.is_err());
                rt.destroy().await.expect("Runtime shuts down");
            }
        }
    };
}

#[macro_export]
macro_rules! test_task_results {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $expectation:expr, $content:literal) => {
        paste::item! {
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;

                let mut ctx = InterContext::default();
                let script = Script::from_text($content, ScriptOptions::strict(), &mut ctx);
                if let Err(err) = &script {
                    eprintln!("{err}");
                }
                script.expect("Script is prepared");
                assert!(ctx.get_diagnostics().expect("Diagnostics available").errors().is_empty());
                let vl = Executor::new(
                    ExecutionOptions::new(
                        $component_name,
                        $task_name,
                        std::env::current_dir().expect("Current folder detected"),
                    ),
                )
                .run(&mut ctx)
                .await;
                if let Err(err) = &vl {
                    eprintln!("{err:?}");
                    if let ExecutorError::Execution(failure) = err {
                        ctx.get_diagnostics().expect("Diagnostics available")
                            .err(failure, &mut std::io::stderr()).expect("Reporting error");
                    }
                }
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
macro_rules! test_task_results_from_file {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $expectation:expr, $filename:literal) => {
        paste::item! {
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                let filepath = std::env::current_dir()
                    .expect("Current folder")
                    .join($filename);
                let mut ctx = InterContext::default();
                let script = Script::from_file(filepath, ScriptOptions::strict(), &mut ctx);
                if let Err(err) = &script {
                    eprintln!("{err}");
                }
                script.expect("Script is prepared");
                assert!(ctx.get_diagnostics().expect("Diagnostics available").errors().is_empty());
                let vl = Executor::new(
                    ExecutionOptions::new(
                        $component_name,
                        $task_name,
                        std::env::current_dir().expect("Current folder detected"),
                    ),
                )
                .run(&mut ctx)
                .await;
                if let Err(err) = &vl {
                    eprintln!("{err:?}");
                    if let ExecutorError::Execution(failure) = err {
                        ctx.get_diagnostics().expect("Diagnostics available")
                            .err(failure, &mut std::io::stderr()).expect("Reporting error");
                    }
                }
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
