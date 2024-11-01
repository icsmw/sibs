

use crate::*;

impl ReadElement<Gatekeeper> for Gatekeeper {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Gatekeeper>, E> {
        Ok(None)
    }
}
