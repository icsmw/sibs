

use crate::*;

impl ReadElement<Meta> for Meta {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Meta>, E> {
        Ok(None)
    }
}
