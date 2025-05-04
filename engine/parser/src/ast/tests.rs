#[macro_export]
macro_rules! test_selfnode_reading {
    ($element_ref:expr, $exp_count:literal) => {
        paste::item! {

            proptest! {
                #![proptest_config(ProptestConfig {
                    max_shrink_iters: 50,
                    ..ProptestConfig::with_cases(500)
                })]

                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $element_ref >](cases in proptest::collection::vec($element_ref::arbitrary(), $exp_count)) {
                    for case in cases.into_iter() {
                        let content = case.to_string();
                        let mut lx = lexer::Lexer::new(&content, 0);
                        let mut parser = $crate::Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &content, false);
                        let node = $element_ref::read(&mut parser);
                        if let Err(err) = &node {
                            eprintln!("{}",parser.report_err(err).expect("Reporting error"));
                            eprintln!("fail with:\nErr:{err:?}\n{content}\n{}", "=".repeat(100));
                        }
                        assert!(node.is_ok());
                        let node = node.unwrap();
                        if node.is_none() {
                            eprintln!("fail with:\n{content}\n{}", "=".repeat(100));
                        }
                        assert!(node.is_some());
                        assert_eq!(node.unwrap().to_string(), content);
                    }
                }

            }
        }
    };
}

#[macro_export]
macro_rules! test_node_reading {
    ($element_ref:expr, $exp_count:literal) => {
        paste::item! {

            proptest! {
                #![proptest_config(ProptestConfig {
                    max_shrink_iters: 50,
                    ..ProptestConfig::with_cases(500)
                })]

                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $element_ref >](cases in proptest::collection::vec($element_ref::arbitrary(), $exp_count)) {
                    for case in cases.into_iter() {
                        let content = case.to_string();
                        let mut lx = lexer::Lexer::new(&content, 0);
                        let mut parser = $crate::Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &content, false);
                        let node = $element_ref::read_as_linked(&mut parser);
                        if let Err(err) = &node {
                            eprintln!("{}",parser.report_err(err).expect("Reporting error"));
                            eprintln!("fail with:\nErr:{err:?}\n{content}\n{}", "=".repeat(100));
                        }
                        assert!(node.is_ok());
                        let node = node.unwrap();
                        if node.is_none() {
                            eprintln!("fail with:\n{content}\n{}", "=".repeat(100));
                        }
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
                    let mut lx = lexer::Lexer::new(&$content, 0);
                    let mut parser = $crate::Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, &$content, false);
                    let node = $element_ref::read_as_linked(&mut parser);
                    if let Err(err) = &node {
                        eprintln!("{}",parser.report_err(err).expect("Reporting error"));
                        eprintln!("fail with:\nErr:{err:?}\n{}\n{}", $content, "=".repeat(100));
                    }
                    assert!(node.is_ok());
                    let node = node.unwrap();
                    assert!(node.is_some());
                    assert_eq!(node.unwrap().to_string(), $content.to_string());
            }
        }
    };
}
