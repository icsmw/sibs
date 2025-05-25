use crate::*;

impl InferType for Error {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Error.into())
    }
}

impl Initialize for Error {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)
    }
}

impl Finalization for Error {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)
    }
}

impl SemanticTokensGetter for Error {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Class,
        )];
        tokens.extend(self.node.get_semantic_tokens(stcx));
        tokens
    }
}
