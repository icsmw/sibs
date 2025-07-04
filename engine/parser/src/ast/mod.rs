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

pub(crate) fn read_and_resolve_nodes(
    parser: &Parser,
    targets: &[NodeTarget],
) -> Result<Option<LinkedNode>, LinkedErr<E>> {
    let mut results = Vec::new();
    let reset = parser.pin();
    for target in targets {
        let drop = parser.pin();
        if let (Some(node), id) = match target {
            NodeTarget::Statement(ids) => (Statement::try_oneof(parser, ids)?, NodeId::Statement),
            NodeTarget::Expression(ids) => {
                (Expression::try_oneof(parser, ids)?, NodeId::Expression)
            }
            NodeTarget::Declaration(ids) => {
                (Declaration::try_oneof(parser, ids)?, NodeId::Declaration)
            }
            NodeTarget::Value(ids) => (Value::try_oneof(parser, ids)?, NodeId::Value),
            NodeTarget::ControlFlowModifier(ids) => (
                ControlFlowModifier::try_oneof(parser, ids)?,
                NodeId::ControlFlowModifier,
            ),
            NodeTarget::Root(ids) => (Root::try_oneof(parser, ids)?, NodeId::Root),
            NodeTarget::Miscellaneous(ids) => (
                Miscellaneous::try_oneof(parser, ids)?,
                NodeId::Miscellaneous,
            ),
        } {
            results.push((parser.pos(), node, id));
        }
        drop(parser);
    }
    reset(parser);
    resolve_conflicts(results, parser)
}

impl TryReadOneOf<LinkedNode, NodeTarget<'_>> for LinkedNode {
    fn try_oneof(
        parser: &Parser,
        targets: &[NodeTarget],
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        let origin = parser.pin();
        let mut shifted = parser.pin();
        loop {
            match read_and_resolve_nodes(parser, targets) {
                Ok(Some(node)) => return Ok(Some(node)),
                Ok(None) => {
                    origin(parser);
                    return Ok(None);
                }
                Err(err) => {
                    if !parser.is_resilience() {
                        return Err(err);
                    }
                    parser.errs.borrow_mut().add(err);
                }
            };
            shifted(parser);
            if parser.token().is_none() {
                origin(parser);
                return Err(parser
                    .errs
                    .borrow_mut()
                    .extract_first()
                    .unwrap_or(LinkedErr::unlinked(E::Unlinked)));
            }
            shifted = parser.pin();
        }
    }
}
