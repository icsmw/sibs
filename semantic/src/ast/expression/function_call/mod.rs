#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionCall {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        let name = self.get_name();
        let Some(entity) = scx.fns.lookup(&name, &self.uuid) else {
            return Err(LinkedErr::between(
                E::FnNotFound(name),
                self.reference.first().map(|(_, t)| t).unwrap_or(&self.open),
                &self.close,
            ));
        };
        Ok(entity.result.clone())
    }
}

impl Initialize for FunctionCall {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for FunctionCall {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let tk_from = self.reference.first().map(|(_, t)| t).unwrap_or(&self.open);
        let p_ty = scx
            .tys
            .get_mut()
            .map_err(|err| LinkedErr::between(err, tk_from, &self.close))?
            .parent
            .withdraw();
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        let mut tys = self
            .args
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(ty) = p_ty {
            tys.insert(0, ty);
        }
        let name = self.get_name();
        let Some(entity) = scx.fns.lookup(&name, &self.uuid) else {
            return Err(LinkedErr::between(
                E::FnNotFound(name),
                tk_from,
                &self.close,
            ));
        };
        if tys.len() != entity.args.len() {
            return Err(LinkedErr::between(
                E::FnArgsNumberDismatch(name, entity.args.len(), tys.len()),
                tk_from,
                &self.close,
            ));
        }
        for (ty, declaration) in tys.iter().zip(entity.args.iter()) {
            if !ty.compatible(&declaration.ty) {
                return Err(LinkedErr::between(
                    E::DismatchTypes(format!("{} and {ty}", declaration.ty)),
                    tk_from,
                    &self.close,
                ));
            }
        }
        Ok(())
    }
}
