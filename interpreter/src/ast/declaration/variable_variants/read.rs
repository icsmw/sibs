use lexer::Kind;

use crate::*;

impl ReadElement<VariableVariants> for VariableVariants {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableVariants>, E> {
        Ok(None)
    }
}
