#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpSeq {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        for node in self
            .nodes
            .iter()
            .filter(|n| !matches!(n.node, Node::Expression(Expression::BinaryOp(..))))
        {
            let ty = node.infer_type(scx)?;
            if !ty.numeric() {
                return Err(LinkedErr::by_node(E::ExpectedNumericType(ty), node));
            }
        }
        Ok(DeterminedTy::Num.into())
    }
}

impl Initialize for BinaryExpSeq {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for BinaryExpSeq {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}
