use crate::*;

impl ReadElement<ComparisonSeq> for ComparisonSeq {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<ComparisonSeq>, E> {
        Ok(None)
    }
}
