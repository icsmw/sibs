use lexer::Kind;

use crate::*;

impl ReadElement<Assignation> for Assignation {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Assignation>, E> {
        Ok(None)
    }
}
