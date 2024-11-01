

use crate::*;

impl ReadElement<Join> for Join {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Join>, E> {
        Ok(None)
    }
}
