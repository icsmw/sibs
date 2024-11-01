

use crate::*;

impl ReadElement<While> for While {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<While>, E> {
        Ok(None)
    }
}
