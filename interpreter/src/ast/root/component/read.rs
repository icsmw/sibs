use lexer::Kind;

use crate::*;

impl ReadElement<Component> for Component {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Component>, E> {
        Ok(None)
    }
}
