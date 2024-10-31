use lexer::Kind;

use crate::*;

impl ReadElement<OneOf> for OneOf {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<OneOf>, E> {
        Ok(None)
    }
}
