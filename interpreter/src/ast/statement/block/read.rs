use lexer::Kind;

use crate::*;

impl ReadElement<Block> for Block {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Block>, E> {
        Ok(None)
    }
}
