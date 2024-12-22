#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionDeclaration {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let mut scx = SemanticCx::default();
        scx.tys.enter(&self.uuid);
        let ty = self.block.infer_type(&mut scx)?;
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err, &self.sig, &self.name))?;
        Ok(ty)
    }
}

impl Initialize for FunctionDeclaration {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let Some(name) = self.get_name() else {
            return Err(LinkedErr::token(E::InvalidFnName, &self.sig));
        };
        scx.tys.enter(&self.uuid);
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        let mut args = Vec::new();
        for n_arg in self.args.iter() {
            let ty = n_arg.infer_type(scx)?;
            args.push(FnArgDeclaration {
                ty,
                link: n_arg.md.link.clone(),
            });
        }
        self.block.initialize(scx)?;
        let entity = FnEntity {
            name: name.to_owned(),
            args,
            result: self.infer_type(scx)?,
            body: FnBody::Node(*self.block.clone()),
        };
        scx.tys
            .leave()
            .map_err(|err| LinkedErr::between(err, &self.sig, &self.name))?;
        scx.fns.add(name, entity).map_err(|err| {
            LinkedErr::between(
                E::FnDeclarationError(err.to_string()),
                &self.sig,
                &self.name,
            )
        })?;
        Ok(())
    }
}
