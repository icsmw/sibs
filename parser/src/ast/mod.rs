mod conflict;
mod metadata;
#[cfg(test)]
mod tests;

mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

use crate::*;
use asttree::*;
use diagnostics::*;

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

pub(crate) fn read_and_resolve_nodes(
    parser: &mut Parser,
    targets: &[NodeReadTarget],
) -> Result<Option<LinkedNode>, LinkedErr<E>> {
    let mut results = Vec::new();
    let reset = parser.pin();
    for target in targets {
        let drop = parser.pin();
        if let (Some(node), id) = match target {
            NodeReadTarget::Statement(ids) => {
                (Statement::try_oneof(parser, ids)?, NodeId::Statement)
            }
            NodeReadTarget::Expression(ids) => {
                (Expression::try_oneof(parser, ids)?, NodeId::Expression)
            }
            NodeReadTarget::Declaration(ids) => {
                (Declaration::try_oneof(parser, ids)?, NodeId::Declaration)
            }
            NodeReadTarget::Value(ids) => (Value::try_oneof(parser, ids)?, NodeId::Value),
            NodeReadTarget::ControlFlowModifier(ids) => (
                ControlFlowModifier::try_oneof(parser, ids)?,
                NodeId::ControlFlowModifier,
            ),
            NodeReadTarget::Root(ids) => (Root::try_oneof(parser, ids)?, NodeId::Root),
            NodeReadTarget::Miscellaneous(ids) => (
                Miscellaneous::try_oneof(parser, ids)?,
                NodeId::Miscellaneous,
            ),
        } {
            results.push((parser.pos, node, id));
        }
        drop(parser);
    }
    reset(parser);
    resolve_conflicts(results, parser)
}

impl TryReadOneOf<LinkedNode, NodeReadTarget<'_>> for LinkedNode {
    fn try_oneof(
        parser: &mut Parser,
        targets: &[NodeReadTarget],
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        let reset = parser.pin();
        Ok(read_and_resolve_nodes(parser, targets)?.or_else(|| {
            reset(parser);
            None
        }))
    }
}
