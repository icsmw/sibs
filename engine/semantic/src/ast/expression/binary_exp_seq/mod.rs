#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpSeq {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        for node in self
            .nodes
            .iter()
            .filter(|n| !matches!(n.get_node(), Node::Expression(Expression::BinaryOp(..))))
        {
            let ty = node.infer_type(scx)?;
            if !ty.numeric() {
                return Err(LinkedErr::from(E::ExpectedNumericType(ty), node));
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

impl SemanticTokensGetter for BinaryExpSeq {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.nodes
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
