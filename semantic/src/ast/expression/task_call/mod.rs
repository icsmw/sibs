#[cfg(test)]
mod tests;

use crate::*;

impl InferType for TaskCall {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let name: String = self.get_name();
        let entity = scx
            .lookup_task(&name, &self.uuid)
            .ok_or(LinkedErr::between(
                E::TaskNotFound(name),
                self.reference.first().map(|(_, t)| t).unwrap_or(&self.open),
                &self.close,
            ))?;
        Ok(entity.result.clone())
    }
}

impl Initialize for TaskCall {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.args.iter().try_for_each(|n| n.initialize(scx))?;
        Ok(())
    }
}

impl Finalization for TaskCall {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        let tk_from = self.reference.first().map(|(_, t)| t).unwrap_or(&self.open);
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        let mut tys = self
            .args
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(ty) = scx
            .tys
            .get()
            .map_err(|err| {
                LinkedErr::between(
                    err.into(),
                    self.reference.first().map(|(_, t)| t).unwrap_or(&self.open),
                    &self.close,
                )
            })?
            .parent
            .get(&self.uuid)
            .cloned()
        {
            tys.insert(0, ty);
        }
        let name: String = self.get_name();
        let entity = scx
            .lookup_task(&name, &self.uuid)
            .ok_or(LinkedErr::between(
                E::TaskNotFound(name.clone()),
                self.reference.first().map(|(_, t)| t).unwrap_or(&self.open),
                &self.close,
            ))?;
        let ts_args = entity.args_tys();
        let mut vl_tys = tys.iter();
        let mut repeated = false;
        for fn_arg in ts_args.iter() {
            if repeated {
                return Err(LinkedErr::between(
                    E::MultipleRepeatedFnArgs,
                    tk_from,
                    &self.close,
                ));
            }
            match fn_arg {
                Ty::Determined(..) | Ty::OneOf(..) | Ty::Variants(..) | Ty::Optional(..) => {
                    let vl_ty = vl_tys.next().ok_or(LinkedErr::between(
                        E::FnArgsNumberDismatch(name.clone(), ts_args.len(), tys.len()),
                        tk_from,
                        &self.close,
                    ))?;
                    let vl_ty = vl_ty.determined().ok_or(LinkedErr::between(
                        E::FailInferDeterminedType(vl_ty.clone()),
                        tk_from,
                        &self.close,
                    ))?;
                    if !match fn_arg {
                        Ty::Determined(arg_ty) | Ty::Variants(arg_ty) | Ty::Optional(arg_ty) => {
                            arg_ty.compatible(vl_ty)
                        }
                        Ty::OneOf(arg_tys) => arg_tys.iter().any(|arg_ty| arg_ty.compatible(vl_ty)),
                        _ => true,
                    } {
                        return Err(LinkedErr::between(
                            E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                            tk_from,
                            &self.close,
                        ));
                    }
                }
                Ty::Repeated(arg_ty) => {
                    repeated = true;
                    for vl_ty in vl_tys.by_ref() {
                        let vl_ty = vl_ty.determined().ok_or(LinkedErr::between(
                            E::FailInferDeterminedType(vl_ty.clone()),
                            tk_from,
                            &self.close,
                        ))?;
                        if !arg_ty.compatible(vl_ty) {
                            return Err(LinkedErr::between(
                                E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                                tk_from,
                                &self.close,
                            ));
                        }
                    }
                }
                Ty::Undefined | Ty::Indeterminate => {
                    return Err(LinkedErr::between(E::InvalidFnArg, tk_from, &self.close));
                }
            }
        }
        if vl_tys.next().is_some() {
            return Err(LinkedErr::between(
                E::FnArgsNumberDismatch(name, ts_args.len(), tys.len()),
                tk_from,
                &self.close,
            ));
        }
        Ok(())
    }
}
