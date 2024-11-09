use crate::*;

impl ReadNode<BinaryExp> for BinaryExp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<BinaryExp>, E> {
        Ok(None)
    }
}
