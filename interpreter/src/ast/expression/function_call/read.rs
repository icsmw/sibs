use lexer::Kind;

use crate::*;

impl ReadElement<FunctionCall> for FunctionCall {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<FunctionCall>, E> {
        Ok(None)
    }
}
