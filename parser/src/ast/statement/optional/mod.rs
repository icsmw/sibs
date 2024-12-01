#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::Kind;

impl ReadNode<Optional> for Optional {
    fn read(parser: &mut Parser) -> Result<Option<Optional>, LinkedErr<E>> {
        let Some(comparison) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::ComparisonSeq])],
        )?
        else {
            return Ok(None);
        };
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::DoubleArrow) {
            return Ok(None);
        }
        let action = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Statement(&[
                    StatementId::Break,
                    StatementId::Return,
                    StatementId::Block,
                    StatementId::Loop,
                    StatementId::For,
                    StatementId::While,
                    StatementId::Assignation,
                    StatementId::Each,
                    StatementId::Join,
                    StatementId::OneOf,
                ]),
                NodeReadTarget::Expression(&[ExpressionId::Command, ExpressionId::FunctionCall]),
            ],
        )?
        .ok_or_else(|| E::MissedActionInOptional.link_with_token(&token))?;
        Ok(Some(Optional {
            token,
            action: Box::new(action),
            comparison: Box::new(comparison),
            uuid: Uuid::new_v4(),
        }))
    }
}
