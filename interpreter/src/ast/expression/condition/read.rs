use lexer::Kind;

use crate::*;

impl ReadElement<Condition> for Condition {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Condition>, E> {
        Ok(None)
    }
}
