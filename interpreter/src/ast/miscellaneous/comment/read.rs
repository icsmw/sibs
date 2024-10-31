use lexer::Kind;

use crate::*;

impl ReadElement<Comment> for Comment {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Comment>, E> {
        Ok(None)
    }
}
