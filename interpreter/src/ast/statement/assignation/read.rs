

use crate::*;

impl ReadNode<Assignation> for Assignation {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Assignation>, E> {
        Ok(None)
    }
}
