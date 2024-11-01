

use crate::*;

impl ReadElement<Comparing> for Comparing {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Comparing>, E> {
        Ok(None)
    }
}
