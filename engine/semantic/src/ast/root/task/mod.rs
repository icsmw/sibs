#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Task {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        let ty = self.block.infer_type(scx)?;
        for gt in self.gts.iter() {
            gt.infer_type(scx)?;
        }
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        Ok(ty)
    }
}

impl Initialize for Task {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(master) = scx.tasks.get_master() else {
            return Err(LinkedErr::sfrom(E::FailToGetMasterOfTask, self));
        };
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.block.initialize(scx)?;
        for gt in self.gts.iter() {
            gt.initialize(scx)?;
        }
        let mut args = Vec::new();
        for n_arg in self.args.iter() {
            let Node::Declaration(Declaration::ArgumentDeclaration(arg_dec)) = n_arg.get_node()
            else {
                return Err(LinkedErr::from(E::InvalidTaskArg, n_arg));
            };
            let Some(ident) = arg_dec.get_var_name() else {
                return Err(LinkedErr::from(E::InvalidTaskArg, n_arg));
            };
            let ty = n_arg.infer_type(scx)?;
            args.push(TaskArgDeclaration {
                ty,
                ident,
                link: n_arg.get_md().link.clone(),
            });
        }
        let entity = TaskEntity {
            uuid: self.uuid,
            name: self.get_name(),
            master,
            args,
            result: match self.infer_type(scx) {
                Ok(ty) => ty,
                Err(_err) => DeterminedTy::Recursion(self.uuid).into(),
            },
            body: TaskBody::Node(self.clone()),
        };
        scx.tasks
            .add(entity)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        Ok(())
    }
}

impl Finalization for Task {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .open(&self.uuid)
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        for gt in self.gts.iter() {
            gt.finalize(scx)?;
        }
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        for arg in self.args.iter() {
            arg.finalize(scx)?;
            if !arg.infer_type(scx)?.is_ty_compatible(&UsageCx::TaskArg) {
                return Err(LinkedErr::sfrom(E::TypeCannotUsedInContext, arg));
            }
        }
        self.block.finalize(scx)?;
        scx.tys
            .close()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?;
        Ok(())
    }
}

impl SemanticTokensGetter for Task {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let mut tokens = vec![
            LinkedSemanticToken::from_token(&self.sig, SemanticToken::Keyword),
            LinkedSemanticToken::from_token(&self.name, SemanticToken::Task),
            LinkedSemanticToken::from_token(&self.open, SemanticToken::Delimiter),
            LinkedSemanticToken::from_token(&self.close, SemanticToken::Delimiter),
        ];
        self.vis
            .as_ref()
            .map(|tk| LinkedSemanticToken::from_token(tk, SemanticToken::Keyword));
        tokens.extend(
            self.args
                .iter()
                .flat_map(|n| n.get_semantic_tokens(SemanticTokenContext::ArgumentDeclaration)),
        );
        tokens.extend(
            self.gts
                .iter()
                .flat_map(|n| n.get_semantic_tokens(SemanticTokenContext::Ignored)),
        );
        tokens.extend(
            self.block
                .get_semantic_tokens(SemanticTokenContext::Ignored),
        );
        tokens
    }
}
