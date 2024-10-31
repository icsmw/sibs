use lexer::Kind;

use crate::*;

impl ReadElement<Meta> for Meta {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Meta>, E> {
        Ok(None)
    }
}
