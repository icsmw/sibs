

use crate::*;

impl ReadElement<ComparingSeq> for ComparingSeq {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<ComparingSeq>, E> {
        Ok(None)
    }
}
