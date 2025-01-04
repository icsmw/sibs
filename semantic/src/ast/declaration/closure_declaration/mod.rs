#[cfg(test)]
mod tests;

use crate::*;

impl InferType for ClosureDeclaration {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let Some((args, out)) = scx.fns.cfns.get_ty(&self.uuid) else {
            return Err(LinkedErr::between(
                E::ClosureNotInited(self.uuid),
                &self.open,
                &self.close,
            ));
        };
        Ok(Ty::Determined(DeterminedTy::Closure(
            self.uuid,
            Some((args, Box::new(out))),
        )))
    }
}

impl Initialize for ClosureDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::between(err.into(), &self.open, &self.close))?;
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
        let entity = ClosureFnEntity {
            uuid: self.uuid,
            args,
            result: match self.ty.infer_type(scx) {
                Ok(ty) => ty,
                Err(_err) => DeterminedTy::Recursion(self.uuid).into(),
            },
            body: FnBody::Declaration,
        };
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err.into(), &self.open, &self.close))?;
        scx.fns.cfns.add(entity).map_err(|err| {
            LinkedErr::between(
                E::FnDeclarationError(err.to_string()),
                &self.open,
                &self.close,
            )
        })?;
        Ok(())
    }
}

impl Finalization for ClosureDeclaration {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        scx.tys
            .enter(&self.uuid)
            .map_err(|err| LinkedErr::between(err.into(), &self.open, &self.close))?;
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        self.ty.initialize(scx)?;
        self.ty.finalize(scx)?;
        let ty = self.ty.infer_type(scx)?;
        scx.fns.cfns.set_result_ty(&self.uuid, ty).map_err(|err| {
            LinkedErr::between(
                E::FnDeclarationError(err.to_string()),
                &self.open,
                &self.close,
            )
        })?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err.into(), &self.open, &self.close))?;
        Ok(())
    }
}
