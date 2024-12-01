#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<While> for While {
    fn read(parser: &mut Parser) -> Result<Option<While>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::While)) {
            return Ok(None);
        }
        let comparison = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[ExpressionId::ComparisonSeq])],
        )?
        .ok_or_else(|| E::MissedComparisonInWhile.link_with_token(&token))?;
        let block =
            LinkedNode::try_oneof(parser, &[NodeReadTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
