#[cfg(test)]
mod tests;

use crate::*;

impl InferType for BinaryExpGroup {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let ty = self.node.infer_type(scx)?;
        if !ty.numeric() {
            Err(LinkedErr::from(E::ExpectedNumericType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for BinaryExpGroup {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for BinaryExpGroup {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}

impl SemanticTokensGetter for BinaryExpGroup {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.node.get_semantic_tokens(stcx)
    }
}
