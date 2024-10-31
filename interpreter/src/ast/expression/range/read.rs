use lexer::Kind;

use crate::*;

impl ReadElement<Range> for Range {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Range>, E> {
        Ok(None)
    }
}
