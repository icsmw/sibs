#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpSeq {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        for node in self
            .nodes
            .iter()
            .filter(|n| !matches!(n.node, Node::Expression(Expression::BinaryOp(..))))
        {
            let ty = node.infer_type(tcx)?;
            if !ty.numeric() {
                return Err(LinkedErr::by_node(E::ExpectedNumericType(ty), node));
            }
        }
        Ok(DataType::Num)
    }
}

impl Initialize for BinaryExpSeq {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(tcx))?;
        self.infer_type(tcx).map(|_| ())
    }
}
