

use crate::*;

impl ReadNode<TaskCall> for TaskCall {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<TaskCall>, E> {
        Ok(None)
    }
}
