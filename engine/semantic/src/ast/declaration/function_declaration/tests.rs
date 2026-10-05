use crate::*;

#[test]
fn shared_function_nodes_have_one_identity_under_multiple_module_paths() {
    use parser::{Parser, TryReadOneOf};

    let source = "mod code { fn helper() { 1; }; fn run() { helper(); }; };";
    let mut lexer = lexer::Lexer::new(source, 0);
    let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false);
    let module = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Module]))
        .unwrap()
        .unwrap();
    let mut scx = SemanticCx::new(false);

    // Reuse the same syntax nodes in both contexts, as a cached module would.
    for path in ["left", "right"] {
        scx.fns.ufns.enter(path);
        module.initialize(&mut scx).unwrap();
        scx.fns.ufns.leave();
    }
    for path in ["left", "right"] {
        scx.fns.ufns.enter(path);
        module.finalize(&mut scx).unwrap();
        scx.fns.ufns.leave();
    }

    assert_eq!(scx.fns.ufns.get_funcs().len(), 2);
    for name in ["helper", "run"] {
        let left = scx.fns.ufns.find(format!("left::code::{name}")).unwrap();
        let right = scx.fns.ufns.find(format!("right::code::{name}")).unwrap();
        assert!(std::ptr::eq(left, right));
    }
    let helper = scx.fns.find("left::code::helper").unwrap();
    assert_eq!(scx.fns.ufns.get_links().len(), 1);
    let caller = scx.fns.ufns.get_links().keys().next().unwrap();
    assert_eq!(
        scx.fns.lookup_by_caller(caller).unwrap().uuid(),
        helper.uuid()
    );
}

test_success!(
    function_declaration_000,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == "one" {
            } if b == 1 {
            } if c == true {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_001,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == "one" {
            } if b == 1 {
            } if c == false {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_002,
    Block,
    r#"{
        fn test(a: str | num, b: num, c: bool) {
            if a == "one" || a == 2 {
            } if b == 1 {
            } if c == false {
            } else {
            };
        }
    }"#
);

test_success!(
    function_declaration_003,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool, cb: |n: num|: num) {
            a;
        }
    }"#
);

test_fail!(
    function_declaration_000,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            if a == 1 {
            } if b == "one" {
            } if c == 12 {
            } else {
            };
        }
    }"#
);

test_fail!(
    function_declaration_001,
    Block,
    r#"{
        fn test(a: str, b: num, c: bool) {
            a = 2;
        }
    }"#
);
