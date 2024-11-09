

use crate::*;

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableDeclaration>, E> {
        Ok(None)
    }
}
