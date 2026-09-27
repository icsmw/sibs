#[macro_export]
macro_rules! test_success {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
            #[test]
            fn [< test_success_ $fn_name >]() {
                use parser::*;
                use $crate::*;
                let mut lx = lexer::Lexer::new(&$content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, $content, false);
                let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                let diagnostics: diagnostics::Diagnostics<ParserError> = parser.try_into().expect("Parser diagnostics are available");
                let mut scx = $crate::SemanticCx::new(false);
                functions::register(&mut scx.fns.efns).expect("functions are registred");
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
            }
        }
    };
}

#[macro_export]
macro_rules! test_fail {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
                #[test]
                fn [< test_finalize_fail_ $fn_name >]() {
                    use parser::*;
                    use $crate::*;
                    let mut lx = lexer::Lexer::new(&$content, 0);
                    let mut parser = Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, $content, false);
                    let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                    let mut scx = $crate::SemanticCx::new(false);
                    functions::register(&mut scx.fns.efns).expect("functions are registred");
                    if node.initialize(&mut scx).is_err() {
                        return;
                    }
                    assert!(node.finalize(&mut scx).is_err());
            }
        }
    };
}
