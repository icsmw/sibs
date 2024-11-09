use crate::*;

impl ReadNode<FunctionCall> for FunctionCall {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<FunctionCall>, E> {
        Ok(None)
    }
}
