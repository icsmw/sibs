

use crate::*;

impl ReadNode<Meta> for Meta {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Meta>, E> {
        Ok(None)
    }
}
