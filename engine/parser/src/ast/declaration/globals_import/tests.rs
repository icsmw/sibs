use crate::ast::tests::Files;
use crate::*;

#[test]
fn relative_imports_preserve_body_identity_and_source() {
    let files = Files::new();
    files.write("globals.sibs", "global count: num = 0;");
    let main = files.write(
        "main.sibs",
        "globals from \"globals.sibs\"; globals from \"./globals.sibs\";",
    );
    let p = Parser::new(main, false).unwrap();
    let anchor = Anchor::read(&p).unwrap().unwrap();
    let first = anchor.nodes[0].extract::<GlobalsImport>().unwrap();
    let second = anchor.nodes[1].extract::<GlobalsImport>().unwrap();
    assert_eq!(first.root.uuid(), second.root.uuid());
    assert_ne!(first.link().src, first.root.link().src);
    let first_body = first.root.extract::<GlobalsModule>().unwrap();
    let second_body = second.root.extract::<GlobalsModule>().unwrap();
    assert_ne!(first_body.nodes[0].uuid(), second_body.nodes[0].uuid());
    assert_eq!(first_body.nodes[0].link(), second_body.nodes[0].link());
}

#[test]
fn repeated_sources_have_independent_tokens_and_bindings() {
    let files = Files::new();
    files.write("body.sibs", "global count: num = 0;");
    let p = Parser::new(files.write("main.sibs", ""), false).unwrap();
    let first = p.from_file("body.sibs").unwrap();
    GlobalsModule::read_as_linked(&first).unwrap().unwrap();
    first.flush().unwrap();
    assert!(first
        .tokens
        .borrow()
        .iter()
        .any(|token| token.owner.is_some()));
    let second = p.from_file("./body.sibs").unwrap();
    assert_eq!(first.src(), second.src());
    assert!(second
        .tokens
        .borrow()
        .iter()
        .all(|token| token.src == second.src()));
    assert!(!Rc::ptr_eq(&first.tokens, &second.tokens));
    assert!(!Rc::ptr_eq(&first.bindings, &second.bindings));
    assert!(second
        .tokens
        .borrow()
        .iter()
        .all(|token| token.owner.is_none()));
    // Registered sources must still be rejected when they are active ancestors.
    assert!(second.from_file("body.sibs").is_err());
    assert!(second.from_file("main.sibs").is_err());
}

#[test]
fn imported_syntax_error_points_to_body() {
    let files = Files::new();
    let body = files.write("body.sibs", "global count = 0;");
    let main = files.write("main.sibs", "globals from \"body.sibs\";");
    let p = Parser::new(main, false).unwrap();
    let error = Anchor::read(&p).unwrap_err();
    assert_ne!(error.link.src, p.src());
    assert!(p
        .get_src_content(Some(&error.link.src))
        .unwrap()
        .unwrap()
        .contains("global count"));
    assert!(body.exists());
}
