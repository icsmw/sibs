#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Loop {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Loop))
    }
}

impl ReadNode<Loop> for Loop {
    fn read(parser: &mut Parser) -> Result<Option<Loop>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Loop)) {
            return Ok(None);
        }
        let block =
            LinkedNode::try_oneof(parser, &[NodeReadTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(Loop {
            token,
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
