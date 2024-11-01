

use crate::*;

impl ReadElement<TaskCall> for TaskCall {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<TaskCall>, E> {
        Ok(None)
    }
}
