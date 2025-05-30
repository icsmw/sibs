use crate::*;

impl InferType for Skip {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Skip {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.func.initialize(scx)
    }
}

impl Finalization for Skip {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        self.func.finalize(scx)
    }
}

impl SemanticTokensGetter for Skip {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Function,
        )];
        tokens.extend(self.func.get_semantic_tokens(stcx));
        tokens.extend(self.args.iter().flat_map(|n| n.get_semantic_tokens(stcx)));
        tokens
    }
}
