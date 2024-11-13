use crate::*;

impl ReadNode<Closure> for Closure {
    fn read(_parser: &mut Parser) -> Result<Option<Closure>, E> {
        Ok(None)
    }
}
