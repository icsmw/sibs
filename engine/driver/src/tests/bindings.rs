use super::*;
use std::collections::HashSet;

fn owners(node: &LinkedNode, ids: &mut HashSet<Uuid>) {
    ids.insert(*node.uuid());
    for child in node.childs() {
        owners(child, ids);
    }
}

#[test]
fn signature_finds_argument_owners_after_speculative_reads() {
    let source = "component comp() { task run() { strs::repeat(\"fd\", 12); } };";
    let mut driver = Driver::unbound(source, false);
    driver.read().unwrap();

    for (text, argument) in [("fd", 0), ("12", 1)] {
        let pos = source.find(text).unwrap();
        let node = driver
            .find_node(pos, None)
            .expect("Argument has an AST owner");
        assert!(matches!(
            node.get_node(),
            Node::Value(Value::PrimitiveString(_) | Value::Number(_))
        ));
        let signature = driver.signature(pos, None).expect("Signature is available");
        assert_eq!(signature.name, "strs::repeat");
        assert_eq!(signature.active, Some(argument));
    }
}

#[test]
fn recovery_keeps_only_owners_from_the_final_ast() {
    let source =
        "component comp() { task run() { strs::repeat(12 true); strs::repeat(\"fd\", 12); } };";
    let mut driver = Driver::unbound(source, true);
    driver.read().unwrap();
    assert!(driver.errors().unwrap().next().is_some());

    let mut ids = HashSet::new();
    owners(driver.ctx.get_anchor().unwrap(), &mut ids);
    let diagnostics = driver.ctx.get_diagnostics().unwrap();
    let mut index = 0;
    while let Some(token) = diagnostics.get_token(index) {
        if let Some((owner, _)) = token.owner {
            assert!(ids.contains(&owner), "Orphan owner for {token}");
        }
        index += 1;
    }
    let signature = driver
        .signature(source.rfind("12").unwrap(), None)
        .expect("Signature survives recovery from the preceding error");
    assert_eq!(signature.name, "strs::repeat");
    assert_eq!(signature.active, Some(1));
}
