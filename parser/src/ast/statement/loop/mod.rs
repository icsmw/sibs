#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<Loop> for Loop {
    fn read(parser: &mut Parser) -> Result<Option<Loop>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Loop)) {
            return Ok(None);
        }
        let block = Statement::try_oneof(parser, &[StatementId::Block])?
            .map(Node::Statement)
            .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(Loop {
            token,
            block: Box::new(block),
        }))
    }
}
