use crate::*;

impl InferType for Comparison {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let left = self.left.infer_type(scx)?;
        let right = self.right.infer_type(scx)?;
        if !left.compatible(&right) {
            Err(LinkedErr::from(
                E::DismatchTypes(format!("{left}, {right}")),
                self,
            ))
        } else {
            Ok(DeterminedTy::Bool.into())
        }
    }
}

impl Initialize for Comparison {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.initialize(scx)?;
        self.operator.initialize(scx)?;
        self.right.initialize(scx)?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Comparison {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.left.finalize(scx)?;
        self.operator.finalize(scx)?;
        self.right.finalize(scx)
    }
}

impl SemanticTokensGetter for Comparison {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = self.left.get_semantic_tokens(stcx);
        tokens.extend(self.operator.get_semantic_tokens(stcx));
        tokens.extend(self.right.get_semantic_tokens(stcx));
        tokens
    }
}
