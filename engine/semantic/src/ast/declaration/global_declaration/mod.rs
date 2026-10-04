mod initializer;

use crate::*;

impl InferType for GlobalDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        self.r#type.infer_type(scx)
    }
}

impl Initialize for GlobalDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variable.initialize(scx)?;
        self.r#type.initialize(scx)?;
        let ty = self.r#type.infer_type(scx)?;
        initializer::validate(&self.assignation, &scx.globals)?;
        self.assignation.initialize(scx)?;
        let actual = self.assignation.infer_type(scx)?;
        if ty != actual {
            return Err(LinkedErr::from(
                E::DismatchTypes(format!("{ty}, {actual}")),
                &self.assignation,
            ));
        }
        let name = self.variable.extract::<VariableName>().ok_or_else(|| {
            LinkedErr::from(
                E::UnexpectedNode(self.variable.get_node().id()),
                &self.variable,
            )
        })?;
        scx.globals
            .register(
                name.ident.clone(),
                GlobalSymbol::new(
                    if self.is_mutable() {
                        GlobalKind::Mutable
                    } else {
                        GlobalKind::Constant
                    },
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

impl Finalization for GlobalDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.variable.finalize(scx)?;
        self.r#type.finalize(scx)?;
        self.assignation.finalize(scx)
    }
}

impl SemanticTokensGetter for GlobalDeclaration {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![LinkedSemanticToken::from_token(
            &self.token,
            SemanticToken::Keyword,
        )];
        for node in self.childs() {
            tokens.extend(node.get_semantic_tokens(SemanticTokenContext::VariableDeclaration));
        }
        tokens
    }
}
