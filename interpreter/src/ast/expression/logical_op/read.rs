use lexer::Kind;

use crate::*;

impl ReadElement<LogicalOp> for LogicalOp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<LogicalOp>, E> {
        Ok(None)
    }
}
