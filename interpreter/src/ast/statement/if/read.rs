

use crate::*;

impl ReadElement<If> for If {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<If>, E> {
        Ok(None)
    }
}
