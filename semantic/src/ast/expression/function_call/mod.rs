#[cfg(test)]
mod tests;

use crate::*;

impl InferType for FunctionCall {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let name = self.get_name();
        if let Some(entity) = scx.fns.lookup(&name, &self.uuid) {
            Ok(entity.result_ty())
        } else if let Some(entity) = scx.fns.efns.lookup(&name, &self.uuid) {
            Ok(Ty::Determined(entity.result.clone()))
        } else {
            Err(LinkedErr::between(
                E::FnNotFound(name),
                self.reference.first().map(|(_, t)| t).unwrap_or(&self.open),
                &self.close,
            ))
        }
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
            .map_err(|err| LinkedErr::between(err.into(), tk_from, &self.close))?
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
        let fn_args = entity.args_tys();
        if tys.len() != fn_args.len() {
            return Err(LinkedErr::between(
                E::FnArgsNumberDismatch(name, fn_args.len(), tys.len()),
                tk_from,
                &self.close,
            ));
        }
        for (ty, dec_ty) in tys.iter().zip(fn_args.iter()) {
            if !ty.compatible(dec_ty) {
                return Err(LinkedErr::between(
                    E::DismatchTypes(format!("{} and {ty}", dec_ty)),
                    tk_from,
                    &self.close,
                ));
            }
        }
        Ok(())
    }
}
