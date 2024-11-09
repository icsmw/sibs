

use crate::*;

impl ReadNode<Closure> for Closure {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Closure>, E> {
        Ok(None)
    }
}
