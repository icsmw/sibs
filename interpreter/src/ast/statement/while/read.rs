use lexer::{Keyword, Kind};

use crate::*;

impl ReadNode<While> for While {
    fn read(parser: &mut Parser) -> Result<Option<While>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::While)) {
            return Ok(None);
        }
        let Some(comparison) =
            Expression::try_oneof(parser, &[ExpressionId::ComparisonSeq])?.map(Node::Expression)
        else {
            return Err(E::MissedComparisonInWhile.link_with_token(&token));
        };
        let Some(block) = Statement::try_oneof(parser, &[StatementId::Block])?.map(Node::Statement)
        else {
            return Err(E::MissedBlock.link_with_token(&token));
        };
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
        }))
    }
}
