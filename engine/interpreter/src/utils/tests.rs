use super::*;
use parser::{Parser, TryReadOneOf};

#[test]
fn converting_functions_to_executors_preserves_aliases_and_call_targets() {
    let source = "mod code { fn helper() { 1; }; };";
    let mut lexer = lexer::Lexer::new(source, 0);
    let parser = Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false);
    let module = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Module]))
        .unwrap()
        .unwrap();
    let mut scx = SemanticCx::new(false);
    module.initialize(&mut scx).unwrap();
    scx.fns.ufns.enter("other");
    module.initialize(&mut scx).unwrap();
    scx.fns.ufns.leave();

    module.finalize(&mut scx).unwrap();
    scx.fns.ufns.enter("other");
    module.finalize(&mut scx).unwrap();
    scx.fns.ufns.leave();
    let uuid = *scx.fns.find("code::helper").unwrap().uuid();
    let first_call = Uuid::new_v4();
    let second_call = Uuid::new_v4();
    scx.fns.lookup("code::helper", &first_call).unwrap();
    scx.fns.lookup("other::code::helper", &second_call).unwrap();

    let fns = into_rt_ufns(scx.fns);
    assert_eq!(fns.ufns.get_funcs().len(), 1);
    let entity = &fns.ufns.get_funcs()[&uuid];
    assert!(matches!(entity.body, UserFnBody::Executor(..)));
    assert!(std::ptr::eq(fns.ufns.find("code::helper").unwrap(), entity));
    assert!(std::ptr::eq(
        fns.ufns.find("other::code::helper").unwrap(),
        entity
    ));
    for caller in [first_call, second_call] {
        assert_eq!(*fns.lookup_by_caller(&caller).unwrap().uuid(), uuid);
    }
}
