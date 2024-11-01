

use crate::*;

impl ReadElement<Condition> for Condition {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Condition>, E> {
        Ok(None)
    }
}
