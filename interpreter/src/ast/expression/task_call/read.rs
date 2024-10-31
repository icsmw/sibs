use lexer::Kind;

use crate::*;

impl ReadElement<TaskCall> for TaskCall {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<TaskCall>, E> {
        Ok(None)
    }
}
