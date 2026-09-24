use crate::*;

impl InferType for RootMeta {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for RootMeta {
    fn initialize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl Finalization for RootMeta {
    fn finalize(&self, _scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        Ok(())
    }
}

impl SemanticTokensGetter for RootMeta {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Meta,
        )]
    }
}
