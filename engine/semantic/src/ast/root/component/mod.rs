use crate::*;

impl InferType for Component {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Component {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tasks.master(self.get_name(), &self.uuid);
        self.nodes.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for Component {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tasks.master(self.get_name(), &self.uuid);
        self.nodes.iter().try_for_each(|n| n.finalize(scx))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Component {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![
            LinkedSemanticToken::from_token(&self.sig, SemanticToken::Keyword),
            LinkedSemanticToken::from_token(&self.name, SemanticToken::Component),
        ];
        tokens.extend(self.nodes.iter().flat_map(|n| n.get_semantic_tokens(stcx)));
        tokens
    }
}
