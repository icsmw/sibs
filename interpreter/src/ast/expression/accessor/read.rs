

use crate::*;

impl ReadElement<Accessor> for Accessor {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Accessor>, E> {
        Ok(None)
    }
}
