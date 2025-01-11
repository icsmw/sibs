use crate::*;

impl InferType for Task {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        scx.tys.open(&self.uuid);
        let ty = self.block.infer_type(scx)?;
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
        scx.tys.open(&self.uuid);
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        self.block.initialize(scx)?;
        let mut args = Vec::new();
        for n_arg in self.args.iter() {
            let Node::Declaration(Declaration::ArgumentDeclaration(arg_dec)) = &n_arg.node else {
                return Err(LinkedErr::from(E::InvalidTaskArg, n_arg));
            };
            let Some(ident) = arg_dec.get_var_name() else {
                return Err(LinkedErr::from(E::InvalidTaskArg, n_arg));
            };
            let ty = n_arg.infer_type(scx)?;
            args.push(TaskArgDeclaration {
                ty,
                ident,
                link: n_arg.md.link.clone(),
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
            body: TaskBody::Node(*self.block.clone()),
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
        scx.tys.open(&self.uuid);
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
