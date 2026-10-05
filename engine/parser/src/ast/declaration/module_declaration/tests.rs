use super::read_body;
use crate::*;
use std::sync::Arc;

struct Files(PathBuf);

impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!("sibs-modules-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn write(&self, name: &str, content: &str) -> PathBuf {
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

fn filename(path: &str) -> PrimitiveString {
    let content = format!("{path:?}");
    let mut lexer = Lexer::new(&content, 0);
    let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, &content, false);
    PrimitiveString::read(&parser).unwrap().unwrap()
}

#[test]
fn loading_reuses_the_body_without_reading_changed_contents() {
    let files = Files::new();
    let root = files.write("main.sibs", "");
    files.write("code.sibs", "fn helper() { 12; };");
    let parser = Parser::new(&root, false).unwrap();
    let first = read_body(&parser, &filename("code.sibs")).unwrap();
    files.write("code.sibs", "this is no longer valid code");
    let second = read_body(&parser, &filename("./code.sibs")).unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(first.nodes.len(), 1);
    assert!(read_body(&Parser::new(root, false).unwrap(), &filename("code.sibs")).is_err());
    let _: Diagnostics<ParserError> = parser.try_into().unwrap();
    assert!(first.nodes[0].extract::<FunctionDeclaration>().is_some());
}

#[test]
fn failed_loads_are_not_cached_and_empty_modules_have_an_identity() {
    let files = Files::new();
    let parser = Parser::new(files.write("main.sibs", ""), false).unwrap();
    files.write("code.sibs", "fn broken(");
    assert!(read_body(&parser, &filename("code.sibs")).is_err());
    files.write("code.sibs", "");
    let first = read_body(&parser, &filename("code.sibs")).unwrap();
    let second = read_body(&parser, &filename("./code.sibs")).unwrap();
    assert!(first.nodes.is_empty());
    assert!(Arc::ptr_eq(&first, &second));
    assert_ne!(first.source, parser.src());
}

#[test]
fn a_cached_body_cannot_bypass_the_active_import_chain() {
    let files = Files::new();
    let parser = Parser::new(files.write("main.sibs", ""), false).unwrap();
    let code = files.write("code.sibs", "");
    read_body(&parser, &filename("code.sibs")).unwrap();
    let child = parser.new_child(code).unwrap();
    let err = read_body(&child, &filename("code.sibs")).unwrap_err();
    assert!(
        matches!(err.e, E::MissedExpectation(_, ref expected) if expected == "acyclic file imports")
    );
}
