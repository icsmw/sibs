#[macro_export]
macro_rules! test_value_expectation {
    ($fn_name:ident, $element_ref:expr, $expectation:expr, $content:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_value_expectation_ $fn_name >]() {
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content);
                let node = $element_ref::read(&mut parser)
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
                let rt = Runtime::new();
                let vl = node.interpret(rt.clone()).await;
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
