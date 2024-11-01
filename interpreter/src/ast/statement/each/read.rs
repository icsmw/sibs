

use crate::*;

impl ReadElement<Each> for Each {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Each>, E> {
        Ok(None)
    }
}
