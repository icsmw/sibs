

use crate::*;

impl ReadElement<Error> for Error {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Error>, E> {
        Ok(None)
    }
}
