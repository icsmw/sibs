// Run block fixtures as task bodies through the same preparation/execution path as scripts.
#[macro_export]
macro_rules! test_value_expectation {
    ($fn_name:ident, Block, $expectation:expr, $content:literal) => {
        $crate::test_task_results!(
            $fn_name,
            "test_component",
            "run",
            $expectation,
            concat!("component test_component() { task run() ", $content, " };")
        );
    };
}

#[macro_export]
macro_rules! test_fail {
    ($fn_name:ident, Block, $content:literal) => {
        paste::item! {
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_fail $fn_name >]() {
                use $crate::*;
                let content = concat!("component test_component() { task run() ", $content, " };");
                let mut ctx = InterContext::default();
                Script::from_text(content, ScriptOptions::strict(), &mut ctx)
                    .expect("Script is prepared before testing an execution failure");
                let result = Executor::new(ExecutionOptions::new(
                    "test_component",
                    "run",
                    std::env::current_dir().expect("Current folder detected"),
                )).run(&mut ctx).await;
                assert!(matches!(result, Err(ExecutorError::Execution(_))), "{result:?}");
            }
        }
    };
}

// A return outside a task/function must execute as an isolated node: wrapping it
// in a task would make the return valid and remove the failure being tested.
#[macro_export]
macro_rules! test_node_fail {
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
                let mut ctx = InterContext::default();
                ctx.set_anchor(node);
                ctx.set_semantic_cx(scx);
                ctx.set_diagnostics(diagnostics.transform(DiagnosticError::from_parser_err));
                let result = Executor::new(ExecutionOptions::new(
                    "test_component",
                    "run",
                    std::env::current_dir().expect("Current folder detected"),
                )).run(&mut ctx).await;
                assert!(matches!(result, Err(ExecutorError::Execution(_))), "{result:?}");
            }
        }
    };
}

#[macro_export]
macro_rules! test_task_results {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $expectation:expr, $content:expr) => {
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
                assert!(!ctx.get_diagnostics().expect("Diagnostics available").has_errors());
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
        $crate::test_task_results_from_file!(
            $fn_name,
            $component_name,
            $task_name,
            $expectation,
            $filename,
            env = []
        );
    };
    ($fn_name:ident, $component_name:literal, $task_name:literal, $expectation:expr, $filename:literal, env = $environment:expr) => {
        $crate::test_task_execution_from_file!(
            $fn_name,
            $component_name,
            $task_name,
            $filename,
            $environment,
            |result: Result<RtValue, ExecutorError>, ctx: &InterContext| {
                if let Err(ExecutorError::Execution(err)) = &result {
                    ctx.get_diagnostics()
                        .expect("Diagnostics available")
                        .err(err, &mut std::io::stderr())
                        .expect("Reporting error");
                }
                let value = result.expect("Task completed");
                assert_eq!(value, $expectation);
            }
        );
    };
}

#[macro_export]
macro_rules! test_task_error_from_file {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $error:pat, $filename:literal, env = $environment:expr) => {
        $crate::test_task_execution_from_file!(
            $fn_name,
            $component_name,
            $task_name,
            $filename,
            $environment,
            |result: Result<RtValue, ExecutorError>, ctx: &InterContext| {
                let Err(ExecutorError::Execution(err)) = result else {
                    panic!("Expected an execution error: {result:?}");
                };
                assert!(matches!(err.e, $error), "{err:?}");
                let mut report = Vec::new();
                ctx.get_diagnostics()
                    .expect("Diagnostics available")
                    .err(&err, &mut report)
                    .expect("Error is linked to a known source");
            }
        );
    };
}

#[macro_export]
macro_rules! test_task_execution_from_file {
    ($fn_name:ident, $component_name:literal, $task_name:literal, $filename:literal, $environment:expr, $check:expr) => {
        paste::item! {
            #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                let name = concat!(module_path!(), "::", stringify!([< test_value_expectation_ $fn_name >]));
                if $crate::tests::run_with_environment(name, &$environment) { return; }
                let filepath = std::env::current_dir().expect("Current folder").join($filename);
                let mut ctx = InterContext::default();
                let script = Script::from_file(filepath, ScriptOptions::strict(), &mut ctx);
                if script.is_err() {
                    if let Some(diagnostics) = ctx.get_diagnostics() {
                        for err in diagnostics.errors() {
                            diagnostics.err(err, &mut std::io::stderr()).expect("Reporting preparation error");
                        }
                    }
                }
                script.expect("Script is prepared");
                assert!(!ctx.get_diagnostics().expect("Diagnostics available").has_errors());
                let result = Executor::new(ExecutionOptions::new(
                    $component_name, $task_name, std::env::current_dir().expect("Current folder"),
                )).run(&mut ctx).await;
                ($check)(result, &ctx);
            }
        }
    };
}
