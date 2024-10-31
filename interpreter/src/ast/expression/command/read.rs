use lexer::Kind;

use crate::*;

impl ReadElement<Command> for Command {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Command>, E> {
        Ok(None)
    }
}
