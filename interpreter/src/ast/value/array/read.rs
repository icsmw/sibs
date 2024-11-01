

use crate::*;

impl ReadElement<Array> for Array {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Array>, E> {
        Ok(None)
    }
}
