use crate::*;

impl ReadNode<Gatekeeper> for Gatekeeper {
    fn read(_parser: &mut Parser) -> Result<Option<Gatekeeper>, E> {
        Ok(None)
    }
}
