

use crate::*;

impl ReadElement<Optional> for Optional {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Optional>, E> {
        Ok(None)
    }
}
