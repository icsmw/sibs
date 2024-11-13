use crate::*;

impl ReadNode<Task> for Task {
    fn read(_parser: &mut Parser) -> Result<Option<Task>, E> {
        Ok(None)
    }
}
