use lexer::Kind;

use crate::*;

impl ReadElement<Gatekeeper> for Gatekeeper {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Gatekeeper>, E> {
        Ok(None)
    }
}
