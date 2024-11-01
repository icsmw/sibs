

use crate::*;

impl ReadElement<Command> for Command {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Command>, E> {
        Ok(None)
    }
}
