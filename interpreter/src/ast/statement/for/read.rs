

use crate::*;

impl ReadNode<For> for For {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<For>, E> {
        Ok(None)
    }
}
