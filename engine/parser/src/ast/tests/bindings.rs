use super::parser;
use crate::*;
use std::collections::HashSet;

fn owners(node: &LinkedNode, ids: &mut HashSet<Uuid>) {
    ids.insert(*node.uuid());
    for child in node.childs() {
        owners(child, ids);
    }
}

fn assert_unowned(parser: &Parser) {
    assert!(parser.tokens.borrow().iter().all(|tk| tk.owner.is_none()));
}

#[test]
fn failed_readers_discard_successful_children() {
    let parser = parser("strs::repeat(\"fd\", 12)");
    assert!(Comparison::read_as_linked(&parser).unwrap().is_none());
    parser.flush().unwrap();
    assert_unowned(&parser);

    let parser = super::parser("strs::repeat(12 true)");
    assert!(FunctionCall::read_as_linked(&parser).is_err());
    parser.flush().unwrap();
    assert_unowned(&parser);
}

#[test]
fn only_the_winning_candidate_owns_tokens_in_either_order() {
    for targets in [
        [
            NodeTarget::Value(&[ValueId::Number]),
            NodeTarget::Expression(&[ExpressionId::BinaryExpSeq]),
        ],
        [
            NodeTarget::Expression(&[ExpressionId::BinaryExpSeq]),
            NodeTarget::Value(&[ValueId::Number]),
        ],
    ] {
        let parser = parser("12");
        let node = LinkedNode::try_oneof(&parser, &targets).unwrap().unwrap();
        assert!(node.extract::<Number>().is_some());
        assert_unowned(&parser);
        parser.flush().unwrap();
        let tokens = parser.tokens.borrow();
        let number = tokens.iter().find(|tk| tk.to_string() == "12").unwrap();
        assert_eq!(number.owner.map(|(id, _)| id), Some(*node.uuid()));
    }
}

#[test]
fn error_in_a_later_alternative_discards_an_earlier_candidate() {
    let parser = parser("12 +");
    assert!(LinkedNode::try_oneof(
        &parser,
        &[
            NodeTarget::Value(&[ValueId::Number]),
            NodeTarget::Expression(&[ExpressionId::BinaryExpSeq]),
        ],
    )
    .is_err());
    parser.flush().unwrap();
    assert_unowned(&parser);
}

#[test]
fn an_outer_attempt_can_discard_a_locally_selected_winner() {
    let parser = parser("12");
    {
        let _attempt = BindingScope::new(parser.bindings.clone());
        LinkedNode::try_read(&parser, NodeTarget::Value(&[ValueId::Number]))
            .unwrap()
            .unwrap();
        assert!(matches!(parser.flush(), Err(E::EarlyFlushCall(_))));
        assert_unowned(&parser);
    }
    parser.flush().unwrap();
    assert_unowned(&parser);
}

#[test]
fn rollback_preserves_previously_accepted_bindings() {
    let parser = parser("7 strs::repeat(\"fd\", 12)");
    let accepted = LinkedNode::try_read(&parser, NodeTarget::Value(&[ValueId::Number]))
        .unwrap()
        .unwrap();
    assert!(Comparison::read_as_linked(&parser).unwrap().is_none());
    parser.flush().unwrap();
    let tokens = parser.tokens.borrow();
    let prefix = tokens.iter().find(|tk| tk.to_string() == "7").unwrap();
    assert_eq!(prefix.owner.map(|(id, _)| id), Some(*accepted.uuid()));
    assert!(tokens
        .iter()
        .filter(|tk| tk.pos.from.abs > prefix.pos.to.abs)
        .all(|tk| tk.owner.is_none()));
}

#[test]
fn borrowed_tokens_do_not_lose_pending_bindings() {
    let parser = parser("12");
    let node = LinkedNode::try_read(&parser, NodeTarget::Value(&[ValueId::Number]))
        .unwrap()
        .unwrap();
    let tokens = parser.tokens.borrow();
    assert!(matches!(parser.flush(), Err(E::EarlyFlushCall(_))));
    assert!(tokens.iter().all(|tk| tk.owner.is_none()));
    drop(tokens);
    parser.flush().unwrap();
    let tokens = parser.tokens.borrow();
    let number = tokens.iter().find(|tk| tk.to_string() == "12").unwrap();
    assert_eq!(number.owner.map(|(id, _)| id), Some(*node.uuid()));
}

#[test]
fn diagnostics_finalize_owners_from_the_accepted_ast_only() {
    for source in [
        "component comp() { task run() { strs::repeat(\"fd\", 12); } };",
        "component comp() { task run() { let value = [1 + 2 * 3, 4]; value; } };",
        "component comp() { task run() { \"fd\".repeat(12); } };",
    ] {
        let parser = parser(source);
        let root = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]))
            .unwrap()
            .unwrap();
        let mut ids = HashSet::new();
        owners(&root, &mut ids);
        assert_unowned(&parser);
        // Conversion must apply the journal even without an explicit flush.
        let diagnostics: Diagnostics<E> = parser.try_into().unwrap();
        let mut owned = 0;
        let store = diagnostics.tokens();
        for token in store.get(store.root()).unwrap().iter() {
            if let Some((owner, _)) = token.owner {
                assert!(ids.contains(&owner), "Orphan owner for {token} in {source}");
                owned += 1;
            }
        }
        assert!(owned > 0);
    }
}

#[test]
fn special_module_readers_roll_back_on_error() {
    let parser = parser("const count: num = 12");
    assert!(GlobalsModule::read_as_linked(&parser).is_err());
    parser.flush().unwrap();
    assert_unowned(&parser);

    let parser = super::parser("required HOME");
    assert!(EnvsModule::read_as_linked(&parser).is_err());
    parser.flush().unwrap();
    assert_unowned(&parser);
}
