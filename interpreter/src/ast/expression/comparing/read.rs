use lexer::Kind;

use crate::*;

impl ReadElement<Comparing> for Comparing {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Comparing>, E> {
        Ok(None)
    }
}
