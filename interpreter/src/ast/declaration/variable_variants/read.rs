

use crate::*;

impl ReadElement<VariableVariants> for VariableVariants {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<VariableVariants>, E> {
        Ok(None)
    }
}
