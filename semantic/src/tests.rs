#[macro_export]
macro_rules! test_success {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
                #[test]
                fn [< test_success $fn_name >]() {
                    use parser::*;
                    use $crate::*;
                    let mut lx = lexer::Lexer::new($content, 0);
                    let tokens = lx.read(true).unwrap().tokens;
                    let mut parser = Parser::new(tokens, &lx.uuid);
                    let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                    let mut tcx = $crate::TypeContext::default();
                    if let Err(err) = node.initialize(&mut tcx) {
                        eprintln!("{err:?}");
                        panic!("{err:?}");
                    }
                    if let Err(err) = node.infer_type(&mut tcx) {
                        eprintln!("{err:?}");
                        panic!("{err:?}");
                    }

            }
        }
    };
}

#[macro_export]
macro_rules! test_fail {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
                #[test]
                fn [< test_fail $fn_name >]() {
                    use parser::*;
                    use $crate::*;
                    let mut lx = lexer::Lexer::new($content, 0);
                    let tokens = lx.read(true).unwrap().tokens;
                    let mut parser = Parser::new(tokens, &lx.uuid);
                    let node = $element_ref::read(&mut parser).expect("Node is parsed without errors").expect("Node is parsed");
                    let mut tcx = $crate::TypeContext::default();
                    assert!(node.initialize(&mut tcx).is_err());
                    assert!(node.infer_type(&mut tcx).is_err());
            }
        }
    };
}
