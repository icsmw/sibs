use crate::*;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum NodeReadTarget<'a> {
    Statement(&'a [StatementId]),
    Expression(&'a [ExpressionId]),
    Declaration(&'a [DeclarationId]),
    Value(&'a [ValueId]),
    ControlFlowModifier(&'a [ControlFlowModifierId]),
    Root(&'a [RootId]),
    Miscellaneous(&'a [MiscellaneousId]),
}

fn select<T: Clone, K: Display + Clone + PartialEq + ConflictResolver<K>>(
    mut results: Vec<(usize, T, K)>,
    parser: &mut Parser,
) -> Result<Option<T>, E> {
    if let Some((n, (pos, tk, id))) = results.iter().enumerate().max_by_key(|(_, (a, ..))| a) {
        let conflicted = results
            .iter()
            .filter(|(p, _, oid)| p == pos && oid != id)
            .cloned()
            .collect::<Vec<(usize, T, K)>>();

        if conflicted.is_empty() {
            parser.pos = *pos;
            Ok(Some(results.remove(n).1))
        } else if let (Some((_, c_tk, c_id)), true) = (conflicted.first(), conflicted.len() == 1) {
            parser.pos = *pos;
            if &id.resolve_conflict(c_id) == id {
                Ok(Some(tk.clone()))
            } else {
                Ok(Some(c_tk.clone()))
            }
        } else {
            Err(E::NodesAreInConflict(
                results
                    .iter()
                    .filter(|(p, ..)| p == pos)
                    .map(|(.., id)| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))
        }
    } else {
        Ok(None)
    }
}

impl Node {
    pub fn try_oneof(
        parser: &mut Parser,
        nodes: &Nodes,
        targets: &[NodeReadTarget],
    ) -> Result<Option<Node>, E> {
        let mut results = Vec::new();
        let reset = parser.pin();
        for target in targets {
            let drop = parser.pin();
            if let (Some(node), id) = match target {
                NodeReadTarget::Statement(ids) => (
                    Statement::try_oneof(parser, ids, nodes)?.map(Node::Statement),
                    NodeId::Statement,
                ),
                NodeReadTarget::Expression(ids) => (
                    Expression::try_oneof(parser, ids, nodes)?.map(Node::Expression),
                    NodeId::Expression,
                ),
                NodeReadTarget::Declaration(ids) => (
                    Declaration::try_oneof(parser, ids, nodes)?.map(Node::Declaration),
                    NodeId::Declaration,
                ),
                NodeReadTarget::Value(ids) => (
                    Value::try_oneof(parser, ids, nodes)?.map(Node::Value),
                    NodeId::Value,
                ),
                NodeReadTarget::ControlFlowModifier(ids) => (
                    ControlFlowModifier::try_oneof(parser, ids, nodes)?
                        .map(Node::ControlFlowModifier),
                    NodeId::ControlFlowModifier,
                ),
                NodeReadTarget::Root(ids) => (
                    Root::try_oneof(parser, ids, nodes)?.map(Node::Root),
                    NodeId::Root,
                ),
                NodeReadTarget::Miscellaneous(ids) => (
                    Miscellaneous::try_oneof(parser, ids, nodes)?.map(Node::Miscellaneous),
                    NodeId::Miscellaneous,
                ),
            } {
                results.push((parser.pos, node, id));
            }

            drop(parser);
        }
        reset(parser);
        resolve_reading_conflicts(results, parser)
    }
}
