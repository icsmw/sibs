use crate::*;

impl InferType for GlobalsImport {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.root.infer_type(scx)
    }
}

impl Initialize for GlobalsImport {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.initialize(scx)?;
        self.root.initialize(scx)
    }
}

impl Finalization for GlobalsImport {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.node.finalize(scx)?;
        self.root.finalize(scx)
    }
}

impl SemanticTokensGetter for GlobalsImport {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![
            LinkedSemanticToken::from_token(&self.sig, SemanticToken::Keyword),
            LinkedSemanticToken::from_token(&self.from, SemanticToken::Keyword),
        ];
        tokens.extend(self.node.get_semantic_tokens(stcx));
        tokens
    }
}
