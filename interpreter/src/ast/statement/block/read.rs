

use crate::*;

impl ReadElement<Block> for Block {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Block>, E> {
        Ok(None)
    }
}
