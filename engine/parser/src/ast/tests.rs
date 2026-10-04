use crate::*;

pub(super) struct Files(PathBuf);
impl Files {
    pub(super) fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-parser-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    pub(super) fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub(super) fn parser(source: &str) -> Parser {
    let mut lexer = Lexer::new(source, 0);
    Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false)
}

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
                            let diagnostics: diagnostics::Diagnostics<$crate::ParserError> = parser.try_into().expect("Parser diagnostics are available");
                            diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
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
                        let tokens = lx.read();
                        if let Err(err) = &tokens {
                            eprintln!("fail with:\nErr:{err:?}\n{content}\n{}", "=".repeat(100));
                        }
                        let mut parser = $crate::Parser::unbound(tokens.unwrap().tokens, &lx.uuid, &content, false);
                        let node = $element_ref::read_as_linked(&mut parser);
                        if let Err(err) = &node {
                            let diagnostics: diagnostics::Diagnostics<$crate::ParserError> = parser.try_into().expect("Parser diagnostics are available");
                            diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
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
macro_rules! test_import_reading {
    ($import:ident, $body:ident, $keyword:literal) => {
        paste::item! {
            proptest! {
                #![proptest_config(ProptestConfig {
                    max_shrink_iters: 50,
                    ..ProptestConfig::with_cases(500)
                })]

                #[allow(non_snake_case)]
                #[test]
                fn [< test_ $import >](body in $body::arbitrary()) {
                    let content = body.to_string();
                    let files = $crate::ast::tests::Files::new();
                    let path = files.write("body.sibs", &content);
                    let source = format!("{} from {:?}", $keyword, path.to_string_lossy());
                    let parser = $crate::ast::tests::parser(&source);
                    let node = $import::read_as_linked(&parser);
                    if let Err(err) = &node {
                        let diagnostics: diagnostics::Diagnostics<$crate::ParserError> = parser.try_into().expect("Parser diagnostics are available");
                        diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                        eprintln!("fail with:\nErr:{err:?}\n{source}\n{content}\n{}", "=".repeat(100));
                        panic!("Import could not be parsed");
                    }
                    let node = node.unwrap().expect("Import node is recognized");
                    prop_assert_eq!(node.to_string(), source);
                    let import = node.extract::<$import>().expect("Expected import node");
                    prop_assert_eq!(import.root.to_string(), content);
                    prop_assert!(parser.is_done());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! test_node_grammar {
    (
        $fn_name:ident, $read:path,
        accepts = [$($accepted:literal),* $(,)?],
        rejects = [$($rejected:literal),* $(,)?] $(,)?
    ) => {
        #[test]
        fn $fn_name() {
            for source in [$($accepted),*] {
                let parser = $crate::ast::tests::parser(source);
                let node = $read(&parser);
                if let Err(err) = &node {
                    let diagnostics: diagnostics::Diagnostics<$crate::ParserError> = parser.try_into().expect("Parser diagnostics are available");
                    diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
                    panic!("Expected valid syntax: {source}\n{err:?}");
                }
                assert!(node.unwrap().is_some(), "Node was not recognized: {source}");
                assert!(parser.is_done(), "Unparsed input remains: {source}");
            }
            for source in [$($rejected),*] {
                let parser = $crate::ast::tests::parser(source);
                assert!($read(&parser).is_err(), "Expected a syntax error: {source}");
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
                        let diagnostics: diagnostics::Diagnostics<$crate::ParserError> = parser.try_into().expect("Parser diagnostics are available");
                            diagnostics.err(err, &mut std::io::stderr()).expect("Reporting error");
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
