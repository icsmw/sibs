use lexer::Kind;

use crate::*;

impl ReadElement<For> for For {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<For>, E> {
        Ok(None)
    }
}
