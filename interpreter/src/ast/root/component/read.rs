

use crate::*;

impl ReadNode<Component> for Component {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Component>, E> {
        Ok(None)
    }
}
