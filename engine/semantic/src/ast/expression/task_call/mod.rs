#[cfg(test)]
mod tests;

use crate::*;

impl InferType for TaskCall {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let name: String = self.get_name();
        let entity = scx
            .lookup_task(&name, &self.uuid)
            .ok_or(LinkedErr::sfrom(E::TaskNotFound(name), self))?;
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
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        let mut tys = self
            .args
            .iter()
            .map(|n| n.infer_type(scx))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(ty) = scx
            .tys
            .get()
            .map_err(|err| LinkedErr::sfrom(err.into(), self))?
            .parent
            .get(&self.uuid)
            .cloned()
        {
            tys.insert(0, ty);
        }
        let name: String = self.get_name();
        let entity = scx
            .lookup_task(&name, &self.uuid)
            .ok_or(LinkedErr::sfrom(E::TaskNotFound(name.clone()), self))?;
        let ts_args = entity.args_tys();
        let mut vl_tys = tys.iter();
        let mut repeated = false;
        for fn_arg in ts_args.iter() {
            if repeated {
                return Err(LinkedErr::sfrom(E::MultipleRepeatedFnArgs, self));
            }
            match fn_arg {
                Ty::Determined(..) | Ty::OneOf(..) | Ty::Variants(..) | Ty::Optional(..) => {
                    let vl_ty = vl_tys.next().ok_or(LinkedErr::sfrom(
                        E::FnArgsNumberDismatch(name.clone(), ts_args.len(), tys.len()),
                        self,
                    ))?;
                    let vl_ty = vl_ty.determined().ok_or(LinkedErr::sfrom(
                        E::FailInferDeterminedType(vl_ty.clone()),
                        self,
                    ))?;
                    if !match fn_arg {
                        Ty::Determined(arg_ty) | Ty::Variants(arg_ty) | Ty::Optional(arg_ty) => {
                            arg_ty.compatible(vl_ty)
                        }
                        Ty::OneOf(arg_tys) => arg_tys.iter().any(|arg_ty| arg_ty.compatible(vl_ty)),
                        _ => true,
                    } {
                        return Err(LinkedErr::sfrom(
                            E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                            self,
                        ));
                    }
                }
                Ty::Repeated(arg_ty) => {
                    repeated = true;
                    for vl_ty in vl_tys.by_ref() {
                        let vl_ty = vl_ty.determined().ok_or(LinkedErr::sfrom(
                            E::FailInferDeterminedType(vl_ty.clone()),
                            self,
                        ))?;
                        if !arg_ty.compatible(vl_ty) {
                            return Err(LinkedErr::sfrom(
                                E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                                self,
                            ));
                        }
                    }
                }
                Ty::Undefined | Ty::Indeterminate => {
                    return Err(LinkedErr::sfrom(E::InvalidFnArg, self));
                }
            }
        }
        if vl_tys.next().is_some() {
            return Err(LinkedErr::sfrom(
                E::FnArgsNumberDismatch(name, ts_args.len(), tys.len()),
                self,
            ));
        }
        Ok(())
    }
}

impl SemanticTokensGetter for TaskCall {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let (Some((_, left)), Some((_, right))) = (self.reference.first(), self.reference.last())
        else {
            return Vec::new();
        };
        let mut tokens = vec![LinkedSemanticToken::between_tokens(
            left,
            right,
            SemanticToken::Task,
        )];
        tokens.extend(
            self.args
                .iter()
                .flat_map(|n| n.get_semantic_tokens(SemanticTokenContext::FunctionCall)),
        );
        tokens
    }
}
