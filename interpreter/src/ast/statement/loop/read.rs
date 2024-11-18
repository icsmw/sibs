use lexer::{Keyword, Kind};

use crate::*;

impl ReadNode<Loop> for Loop {
    fn read(parser: &mut Parser) -> Result<Option<Loop>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Loop)) {
            return Ok(None);
        }
        let Some(block) = Statement::try_oneof(parser, &[StatementId::Block])?.map(Node::Statement)
        else {
            return Err(E::MissedBlock.link_with_token(&token));
        };
        Ok(Some(Loop {
            token,
            block: Box::new(block),
        }))
    }
}
