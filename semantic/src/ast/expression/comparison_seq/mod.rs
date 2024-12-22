#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ComparisonSeq {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        for node in self
            .nodes
            .iter()
            .filter(|n| !matches!(n.node, Node::Expression(Expression::LogicalOp(..))))
        {
            let ty = node.infer_type(scx)?;
            if !matches!(ty, DataType::Bool) {
                return Err(LinkedErr::by_node(E::ExpectedBoolType(ty), node));
            }
        }
        Ok(DataType::Bool)
    }
}

impl Initialize for ComparisonSeq {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for ComparisonSeq {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
