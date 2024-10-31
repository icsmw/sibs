use lexer::Kind;

use crate::*;

impl ReadElement<VariableType> for VariableType {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableType>, E> {
        Ok(None)
    }
}
