

use crate::*;

impl ReadElement<BinaryExp> for BinaryExp {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<BinaryExp>, E> {
        Ok(None)
    }
}
