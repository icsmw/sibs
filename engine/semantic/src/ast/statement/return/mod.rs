use crate::*;

impl InferType for Return {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.node
            .as_ref()
            .map(|n| n.infer_type(scx))
            .unwrap_or_else(|| Ok(DeterminedTy::Void.into()))
    }
}

impl Initialize for Return {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.node.as_ref() {
            n.initialize(scx)?;
            n.infer_type(scx).map(|_| ())?;
        }
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Return {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if let Some(n) = self.node.as_ref() {
            n.finalize(scx)?;
        }
        Ok(())
    }
}

impl SemanticTokensGetter for Return {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Keyword,
        )];
        self.node
            .as_ref()
            .map(|n| tokens.extend(n.get_semantic_tokens(stcx)));
        tokens
    }
}
