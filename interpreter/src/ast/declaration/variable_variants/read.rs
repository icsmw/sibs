use crate::*;

impl ReadNode<VariableVariants> for VariableVariants {
    fn read(_parser: &mut Parser) -> Result<Option<VariableVariants>, E> {
        Ok(None)
    }
}
