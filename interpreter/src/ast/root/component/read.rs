use crate::*;

impl ReadNode<Component> for Component {
    fn read(_parser: &mut Parser) -> Result<Option<Component>, E> {
        Ok(None)
    }
}
