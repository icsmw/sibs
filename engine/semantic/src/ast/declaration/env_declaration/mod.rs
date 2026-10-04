use crate::*;

impl InferType for EnvDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Str.into())
    }
}

impl Initialize for EnvDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variable.initialize(scx)?;
        let name = self.variable.extract::<VariableName>().ok_or_else(|| {
            LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            )
        })?;
        let ty = self.infer_type(scx)?;
        scx.globals
            .register(
                name.ident.clone(),
                GlobalSymbol::new(
                    GlobalKind::Environment,
                    self.uuid,
                    self.variable.link(),
                    ty.clone(),
                ),
            )
            .map_err(|err| LinkedErr::from(err.into(), &self.variable))?;
        scx.link_ty_with_node(self.variable.uuid(), ty);
        Ok(())
    }
}

impl Finalization for EnvDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variable.finalize(scx)
    }
}

impl SemanticTokensGetter for EnvDeclaration {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Keyword,
        )];
        tokens.extend(
            self.variable
                .get_semantic_tokens(SemanticTokenContext::VariableDeclaration),
        );
        tokens
    }
}
