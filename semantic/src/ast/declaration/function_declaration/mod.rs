#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::token(err.into(), &self.sig))?;
        let ty = self.block.infer_type(scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err.into(), &self.sig, &self.name))?;
        Ok(ty)
    }
}

impl Initialize for FunctionDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::token(E::InvalidFnName, &self.sig));
        };
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::token(err.into(), &self.sig))?;
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        let mut args = Vec::new();
        for n_arg in self.args.iter() {
            let Node::Declaration(Declaration::ArgumentDeclaration(arg_dec)) = &n_arg.node else {
                return Err(LinkedErr::by_node(E::InvalidFnArg, n_arg));
            };
            let Some(ident) = arg_dec.get_var_name() else {
                return Err(LinkedErr::by_node(E::InvalidFnArg, n_arg));
            };
            let ty = n_arg.infer_type(scx)?;
            args.push(UserFnArgDeclaration {
                ty,
                ident,
                link: n_arg.md.link.clone(),
            });
        }
        let entity = UserFnEntity {
            uuid: self.uuid,
            name: name.to_owned(),
            fullname: name.to_owned(),
            args,
            result: match self.infer_type(scx) {
                Ok(ty) => ty,
                Err(_err) => DeterminedTy::Recursion(self.uuid).into(),
            },
            body: FnBody::Node(*self.block.clone()),
        };
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err.into(), &self.sig, &self.name))?;
        scx.fns.ufns.add(name, entity).map_err(|err| {
            LinkedErr::between(
                E::FnDeclarationError(err.to_string()),
                &self.sig,
                &self.name,
            )
        })?;
        Ok(())
    }
}

impl Finalization for FunctionDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        // let Some(name) = self.get_name() else {
        //     return Err(LinkedErr::token(E::InvalidFnName, &self.sig));
        // };
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::token(err.into(), &self.sig))?;
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        // Initialization of fn's block cannot be done in the scope of `Initialize` because
        // it might fall into recursion
        self.block.initialize(scx)?;
        self.block.finalize(scx)?;
        // This block is useless. Has to be removed after confirming
        // let ty = self.infer_type(scx)?;
        // scx.fns.ufns.set_result_ty(name, ty).map_err(|err| {
        //     LinkedErr::between(
        //         E::FnDeclarationError(err.to_string()),
        //         &self.sig,
        //         &self.name,
        //     )
        // })?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err.into(), &self.sig, &self.name))?;
        Ok(())
    }
}
