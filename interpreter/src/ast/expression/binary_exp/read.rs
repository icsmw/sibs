use lexer::Kind;

use crate::*;

impl ReadElement<BinaryExp> for BinaryExp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<BinaryExp>, E> {
        Ok(None)
    }
}
