pub const PROPTEST_DEEP_FACTOR: u8 = 5;

#[macro_export]
macro_rules! test_node_reading {
    ($fn_name:ident, $element_ref:expr, $exp_count:literal) => {
        paste::item! {

            proptest! {
                #![proptest_config(ProptestConfig {
                    max_shrink_iters: 50,
                    ..ProptestConfig::with_cases(500)
                })]

                #[test]
                fn [< test_ $fn_name >](cases in proptest::collection::vec($element_ref::arbitrary(), $exp_count)) {
                    for case in cases.into_iter() {
                        let content = case.to_string();
                        let mut lx = lexer::Lexer::new(&content, 0);
                        let mut parser = $crate::Parser::new(lx.read(true).unwrap().tokens);
                        let node = $element_ref::read(&mut parser).unwrap();
                        assert!(node.is_some());
                        assert_eq!(node.unwrap().to_string(), content);
                    }
                }

            }
        }
    };
}

#[macro_export]
macro_rules! test_node_reading_case {
    ($fn_name:ident, $element_ref:expr, $content:literal) => {
        paste::item! {
                #[test]
                fn [< test_ $fn_name >]() {
                    let mut lx = lexer::Lexer::new($content, 0);
                    let tokens = lx.read(true).unwrap().tokens;
                    println!(">>>>>>>>>>>>>>>>>>>>:{tokens:?}");
                    let mut parser = $crate::Parser::new(tokens);
                    let node = $element_ref::read(&mut parser).unwrap();
                    assert!(node.is_some());
                    assert_eq!(node.unwrap().to_string(), $content.to_string());
            }
        }
    };
}
