

use crate::*;

impl ReadNode<Loop> for Loop {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Loop>, E> {
        Ok(None)
    }
}
