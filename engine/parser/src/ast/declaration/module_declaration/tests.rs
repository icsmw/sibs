use super::read_body;
use crate::*;
use std::sync::Arc;
use test_utils::Files;

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
    let diagnostics: Diagnostics<ParserError> = parser.try_into().unwrap();
    let store = diagnostics.tokens();
    assert_eq!(store.iter().count(), 2);
    assert!(store
        .get(&first.source)
        .unwrap()
        .iter()
        .any(|token| token.to_string() == "12"));
    assert!(first.nodes[0].extract::<FunctionDeclaration>().is_some());
}

#[test]
fn a_new_parse_can_load_a_fixed_empty_module() {
    let files = Files::new();
    let root = files.write("main.sibs", "");
    let parser = Parser::new(&root, false).unwrap();
    files.write("code.sibs", "fn broken(");
    assert!(read_body(&parser, &filename("code.sibs")).is_err());
    drop(parser);
    files.write("code.sibs", "");
    let parser = Parser::new(root, false).unwrap();
    let first = read_body(&parser, &filename("code.sibs")).unwrap();
    let second = read_body(&parser, &filename("./code.sibs")).unwrap();
    assert!(first.nodes.is_empty());
    assert!(Arc::ptr_eq(&first, &second));
    assert_ne!(first.source, parser.src());
    let diagnostics: Diagnostics<ParserError> = parser.try_into().unwrap();
    let tokens = diagnostics.tokens().get(&first.source).unwrap();
    assert!(tokens
        .iter()
        .all(|token| matches!(token.id(), KindId::BOF | KindId::EOF)));
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
