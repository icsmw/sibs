use crate::*;

impl ReadNode<Meta> for Meta {
    fn read(_parser: &mut Parser) -> Result<Option<Meta>, E> {
        Ok(None)
    }
}
