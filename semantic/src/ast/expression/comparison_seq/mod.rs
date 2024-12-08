#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ComparisonSeq {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        for node in self
            .nodes
            .iter()
            .filter(|n| !matches!(n.node, Node::Expression(Expression::LogicalOp(..))))
        {
            let ty = node.infer_type(tcx)?;
            if !matches!(ty, DataType::Bool) {
                return Err(LinkedErr::by_node(E::ExpectedBoolType(ty), node));
            }
        }
        Ok(DataType::Bool)
    }
}

impl Initialize for ComparisonSeq {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        self.infer_type(tcx).map(|_| ())
    }
}
