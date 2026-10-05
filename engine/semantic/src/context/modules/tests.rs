use crate::tests::Files;
use crate::*;
use parser::{Parser, TryReadOneOf};

#[test]
fn shared_body_analysis_is_local_to_the_context_and_does_not_repeat_finalization() {
    let files = Files::new();
    files.write(
        "library.sibs",
        "fn helper(n: num) { let cb = |v: num| { v; }; n; };",
    );
    let parser = Parser::new(
        files.write("main.sibs", "mod from \"library.sibs\";"),
        false,
    )
    .unwrap();
    let node = LinkedNode::try_read(
        &parser,
        NodeTarget::Declaration(&[DeclarationId::ModuleDeclaration]),
    )
    .unwrap()
    .unwrap();
    let source = node.extract::<ModuleDeclaration>().unwrap().body.source;

    // Both contexts analyze exactly the same AST and shared module body.
    for _ in 0..2 {
        let mut scx = SemanticCx::new(false);
        assert!(!scx.modules.is_initialized(&source));
        assert!(!scx.modules.is_finalized(&source));

        node.initialize(&mut scx).unwrap();
        assert!(scx.modules.is_initialized(&source));
        assert!(!scx.modules.is_finalized(&source));
        assert!(scx.fns.find("library::helper").is_some());

        node.finalize(&mut scx).unwrap();
        assert!(scx.modules.is_initialized(&source));
        assert!(scx.modules.is_finalized(&source));
        assert_eq!(scx.fns.cfns.get_funcs().len(), 1);

        // Another initialization must not downgrade the completed stage or
        // cause the function's closure to be registered a second time.
        node.initialize(&mut scx).unwrap();
        assert!(scx.modules.is_finalized(&source));
        node.finalize(&mut scx).unwrap();
        assert_eq!(scx.fns.cfns.get_funcs().len(), 1);
    }
}
