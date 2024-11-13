use crate::*;

impl ReadNode<Comment> for Comment {
    fn read(_parser: &mut Parser) -> Result<Option<Comment>, E> {
        Ok(None)
    }
}
