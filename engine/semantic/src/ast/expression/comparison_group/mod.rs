#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ComparisonGroup {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let ty = self.node.infer_type(scx)?;
        if !ty.bool() {
            Err(LinkedErr::from(E::ExpectedBoolType(ty), &self.node))
        } else {
            Ok(ty)
        }
    }
}

impl Initialize for ComparisonGroup {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for ComparisonGroup {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}

impl SemanticTokensGetter for ComparisonGroup {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.node.get_semantic_tokens(stcx)
    }
}
