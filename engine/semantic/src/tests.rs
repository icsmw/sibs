use crate::*;
use parser::{Parser, TryReadOneOf};
use std::path::PathBuf;

pub(crate) fn analyze_globals(source: &str) -> Result<SemanticCx, LinkedErr<E>> {
    let files = Files::new();
    files.write("globals.sibs", source);
    files.analyze("globals from \"globals.sibs\";")
}

pub(crate) struct Files(pub(crate) PathBuf);
impl Files {
    pub(crate) fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-semantic-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    pub(crate) fn write(&self, name: &str, content: &str) -> PathBuf {
        let path = self.0.join(name);
        std::fs::write(&path, content).unwrap();
        path
    }
    pub(crate) fn analyze(&self, source: &str) -> Result<SemanticCx, LinkedErr<E>> {
        let parser = Parser::new(self.write("main.sibs", source), false).unwrap();
        let node = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]))
            .unwrap()
            .unwrap();
        let mut scx = SemanticCx::new(false);
        node.initialize(&mut scx)?;
        node.infer_type(&mut scx)?;
        node.finalize(&mut scx)?;
        Ok(scx)
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[macro_export]
macro_rules! test_semantic_cases {
    (
        $group:ident,
        |$($parameter:ident: $ty:ty),* $(,)?| $checks:block,
        $($case:ident => ($($argument:expr),* $(,)?)),+ $(,)?
    ) => {
        mod $group {
            use super::*;

            fn check($($parameter: $ty),*) $checks

            $(
                #[test]
                fn $case() {
                    check($($argument),*);
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! test_semantic_files {
    (
        $fn_name:ident,
        files = {$($filename:literal => $content:expr),* $(,)?},
        |$files:ident| $checks:block
    ) => {
        #[test]
        fn $fn_name() {
            let $files = $crate::tests::Files::new();
            $($files.write($filename, $content);)*
            $checks
        }
    };
}

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
