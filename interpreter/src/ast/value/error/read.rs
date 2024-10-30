use lexer::Kind;

use crate::*;

impl ReadElement<Error> for Error {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Error>, E> {
        Ok(None)
    }
}
