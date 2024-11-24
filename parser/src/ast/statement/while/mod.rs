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
        let comparison = Expression::try_oneof(parser, &[ExpressionId::ComparisonSeq])?
            .map(Node::Expression)
            .ok_or_else(|| E::MissedComparisonInWhile.link_with_token(&token))?;
        let block = Statement::try_oneof(parser, &[StatementId::Block])?
            .map(Node::Statement)
            .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
