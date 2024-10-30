use lexer::Kind;

use crate::*;

impl ReadElement<Command> for Command {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Command>, E> {
        if let Some(tk) = parser.token() {
            let Kind::Command(_) = &tk.kind else {
                return Ok(None);
            };
        }
        Ok(None)
    }
}
