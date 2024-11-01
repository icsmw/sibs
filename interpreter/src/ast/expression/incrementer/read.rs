use crate::*;

impl ReadElement<Incrementer> for Incrementer {
    fn read(_parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Incrementer>, E> {
        Ok(None)
    }
}
