

use crate::*;

impl ReadNode<Comment> for Comment {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Comment>, E> {
        Ok(None)
    }
}
