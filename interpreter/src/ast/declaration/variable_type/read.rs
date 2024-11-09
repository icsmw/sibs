

use crate::*;

impl ReadNode<VariableType> for VariableType {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableType>, E> {
        Ok(None)
    }
}
