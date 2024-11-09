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
