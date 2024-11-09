

use crate::*;

impl ReadNode<Task> for Task {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Task>, E> {
        Ok(None)
    }
}
