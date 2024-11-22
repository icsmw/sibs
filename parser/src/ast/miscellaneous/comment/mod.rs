#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::Kind;

impl ReadNode<Comment> for Comment {
    fn read(parser: &mut Parser) -> Result<Option<Comment>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Comment(..)) {
            return Ok(None);
        }
        Ok(Some(Comment { token }))
    }
}
