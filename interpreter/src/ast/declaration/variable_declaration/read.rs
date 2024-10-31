use lexer::Kind;

use crate::*;

impl ReadElement<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableDeclaration>, E> {
        Ok(None)
    }
}
