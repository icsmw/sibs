#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for While {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::While))
    }
}

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
            &[
                NodeTarget::Expression(&[ExpressionId::ComparisonSeq, ExpressionId::Variable]),
                NodeTarget::Value(&[ValueId::Boolean]),
            ],
        )?
        .ok_or_else(|| E::MissedComparisonInWhile.link_with_token(&token))?;
        let block =
            LinkedNode::try_oneof(parser, &[NodeTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
