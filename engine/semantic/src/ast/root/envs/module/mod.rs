use crate::*;

impl InferType for EnvsModule {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for EnvsModule {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        for node in self.nodes.iter() {
            node.initialize(scx)?;
        }
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for EnvsModule {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        for node in self.nodes.iter() {
            node.finalize(scx)?;
        }
        Ok(())
    }
}

impl SemanticTokensGetter for EnvsModule {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        self.nodes
            .iter()
            .flat_map(|n| n.get_semantic_tokens(stcx))
            .collect()
    }
}
