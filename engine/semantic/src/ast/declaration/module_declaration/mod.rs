use crate::*;

impl InferType for ModuleDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        Ok(DeterminedTy::Void.into())
    }
}

impl Initialize for ModuleDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .open(&self.body.source)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(&self.name);
        let result = if scx.modules.is_initialized(&self.body.source) {
            expose_functions(&self.body.nodes, scx)
        } else {
            self.body
                .nodes
                .iter()
                .try_for_each(|node| node.initialize(scx))
        };
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        result?;
        scx.modules.initialized(self.body.source);
        self.infer_type(scx).map(|_| ())
    }
}

impl Finalization for ModuleDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        if scx.modules.is_finalized(&self.body.source) {
            return Ok(());
        }
        scx.tys
            .open(&self.body.source)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.fns.ufns.enter(&self.name);
        let result = self
            .body
            .nodes
            .iter()
            .try_for_each(|node| node.finalize(scx));
        scx.fns.ufns.leave();
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        result?;
        scx.modules.finalized(self.body.source);
        Ok(())
    }
}

// The body has already been initialized. Only its exported paths are new;
// types, globals and function bodies retain their original identities.
fn expose_functions(nodes: &[LinkedNode], scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
    for node in nodes {
        match node.get_node() {
            Node::Declaration(Declaration::FunctionDeclaration(function)) => {
                // In resilient mode a failed declaration may have no entity.
                if !scx.fns.ufns.get_funcs().contains_key(&function.uuid) {
                    continue;
                }
                let name = function
                    .get_name()
                    .ok_or_else(|| LinkedErr::from(E::InvalidFnName, function))?;
                scx.fns
                    .ufns
                    .add_alias(name, &function.uuid)
                    .map_err(|err| {
                        LinkedErr::from(E::FnDeclarationError(err.to_string()), function)
                    })?;
            }
            Node::Declaration(Declaration::ModuleDeclaration(module)) => {
                scx.fns.ufns.enter(&module.name);
                let result = expose_functions(&module.body.nodes, scx);
                scx.fns.ufns.leave();
                result?;
            }
            Node::Root(Root::Module(module)) => {
                let name = module
                    .get_name()
                    .ok_or_else(|| LinkedErr::from(E::InvalidModuleName, module))?;
                scx.fns.ufns.enter(name);
                let result = expose_functions(&module.nodes, scx);
                scx.fns.ufns.leave();
                result?;
            }
            _ => {}
        }
    }
    Ok(())
}

impl SemanticTokensGetter for ModuleDeclaration {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![
            LinkedSemanticToken::from_token(&self.sig, SemanticToken::Keyword),
            LinkedSemanticToken::from_token(&self.from, SemanticToken::Keyword),
        ];
        tokens.extend(self.node.get_semantic_tokens(stcx));
        tokens
    }
}
