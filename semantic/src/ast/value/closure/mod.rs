#[cfg(test)]
mod tests;

use crate::*;

impl InferType for Closure {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Some((args, out)) = scx.fns.cfns.get_ty(&self.uuid) else {
            return Err(LinkedErr::from(E::ClosureNotInited(self.uuid), self));
        };
        Ok(Ty::Determined(DeterminedTy::Closure(
            self.uuid,
            Some((args, Box::new(out))),
        )))
    }
}

impl Initialize for Closure {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        let mut args = Vec::new();
        for n_arg in self.args.iter() {
            let Node::Declaration(Declaration::ArgumentDeclaration(arg_dec)) = &n_arg.node else {
                return Err(LinkedErr::from(E::InvalidFnArg, n_arg));
            };
            let Some(ident) = arg_dec.get_var_name() else {
                return Err(LinkedErr::from(E::InvalidFnArg, n_arg));
            };
            let ty = n_arg.infer_type(scx)?;
            args.push(UserFnArgDeclaration {
                ty,
                ident,
                link: n_arg.md.link.clone(),
            });
        }
        let entity = ClosureFnEntity {
            uuid: self.uuid,
            args,
            result: match self.block.infer_type(scx) {
                Ok(ty) => ty,
                Err(_err) => DeterminedTy::Recursion(self.uuid).into(),
            },
            body: FnBody::Node(*self.block.clone()),
        };
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        scx.fns
            .cfns
            .add(entity)
            .map_err(|err| LinkedErr::from(E::FnDeclarationError(err.to_string()), self))?;
        Ok(())
    }
}

impl Finalization for Closure {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        // Initialization of fn's block cannot be done in the scope of `Initialize` because
        // it might fall into recursion
        self.block.initialize(scx)?;
        self.block.finalize(scx)?;
        let ty = self.block.infer_type(scx)?;
        scx.fns
            .cfns
            .set_result_ty(&self.uuid, ty)
            .map_err(|err| LinkedErr::from(E::FnDeclarationError(err.to_string()), self))?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::from(err.into(), self))?;
        Ok(())
    }
}
