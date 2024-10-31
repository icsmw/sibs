use lexer::Kind;

use crate::*;

impl ReadElement<ComparingSeq> for ComparingSeq {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<ComparingSeq>, E> {
        Ok(None)
    }
}
