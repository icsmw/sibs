

use crate::*;

impl ReadNode<OneOf> for OneOf {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<OneOf>, E> {
        Ok(None)
    }
}
