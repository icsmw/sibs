use crate::*;

impl InferType for Module {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for Module {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::sfrom(E::InvalidModuleName, self));
        };
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(name);
        for node in self.nodes.iter() {
            node.initialize(scx)?;
        }
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for Module {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::sfrom(E::InvalidModuleName, self));
        };
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(name);
        for node in self.nodes.iter() {
            node.finalize(scx)?;
        }
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Module {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![
            LinkedSemanticToken::from_token(&self.sig, SemanticToken::Keyword),
            LinkedSemanticToken::from_token(&self.name, SemanticToken::Module),
            LinkedSemanticToken::from_token(&self.open, SemanticToken::Delimiter),
            LinkedSemanticToken::from_token(&self.close, SemanticToken::Delimiter),
        ];
        tokens.extend(self.nodes.iter().flat_map(|n| n.get_semantic_tokens(stcx)));
        tokens
    }
}
