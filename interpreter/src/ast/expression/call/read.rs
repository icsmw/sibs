

use crate::*;

impl ReadElement<Call> for Call {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Call>, E> {
        Ok(None)
    }
}
