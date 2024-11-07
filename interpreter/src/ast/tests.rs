#[macro_export]
macro_rules! test_node_reading {
    ($fn_name:ident, $element_ref:expr, $exp_count:literal) => {
        paste::item! {
            use proptest::prelude::*;

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
                        assert!($element_ref::read(&mut parser, &$crate::Nodes::empty()).unwrap().is_some());
                    }
                }

            }
        }
    };
}
