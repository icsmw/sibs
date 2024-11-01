

use crate::*;

impl ReadElement<LogicalOp> for LogicalOp {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<LogicalOp>, E> {
        Ok(None)
    }
}
