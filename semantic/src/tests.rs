#[macro_export]
macro_rules! test_success {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
            #[test]
            fn [< test_success_ $fn_name >]() {
                use parser::*;
                use $crate::*;
                let mut lx = lexer::Lexer::new($content, 0);
                let mut parser = Parser::unbound(lx.read().unwrap().tokens);
                let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                let mut scx = $crate::SemanticCx::default();
                let result = node.initialize(&mut scx);
                if let Err(err) = &result {
                    eprintln!("{err:?}");
                }
                assert!(result.is_ok());
                let result = node.infer_type(&mut scx);
                if let Err(err) = &result {
                    eprintln!("{err:?}");
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
                fn [< test_fail_ $fn_name >]() {
                    use parser::*;
                    use $crate::*;
                    let mut lx = lexer::Lexer::new($content, 0);
                    let mut parser = Parser::unbound(lx.read().unwrap().tokens);
                    let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                    let mut scx = $crate::SemanticCx::default();
                    assert!(node.initialize(&mut scx).is_err());
            }
        }
    };
}
