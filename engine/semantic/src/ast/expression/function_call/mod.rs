#[cfg(test)]
mod tests;

use crate::*;

fn get_args_tys(node: &FunctionCall, scx: &mut SemanticCx) -> Result<Vec<Ty>, LinkedErr<E>> {
    let mut tys = node
        .args
        .iter()
        .map(|n| n.infer_type(scx))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(ty) = scx
        .tys
        .get()
        .map_err(|err| LinkedErr::from(err.into(), node))?
        .parent
        .get(&node.uuid)
        .cloned()
    {
        tys.insert(0, ty);
    }
    Ok(tys)
}

impl InferType for FunctionCall {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        let name: String = self.get_name();
        if let Some(entity) = scx
            .lookup_fn(&name, &self.uuid)
            .map_err(|err| LinkedErr::from(err, self))?
        {
            Ok(entity.result_ty())
        } else {
            let last_name: String = self.get_last_name();
            let tys = get_args_tys(self, scx)?;
            if let Some(entity) =
                scx.lookup_fn_by_inps(&last_name, &tys.iter().collect::<Vec<&Ty>>(), &self.uuid)
            {
                Ok(entity.result_ty())
            } else {
                Err(LinkedErr::from(E::FnNotFound(name), self))
            }
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
        self.args.iter().try_for_each(|n| n.finalize(scx))?;
        let tys = get_args_tys(self, scx)?;
        let name = self.get_name();
        let Some(entity) = (if let Some(entity) = scx
            .lookup_fn(&name, &self.uuid)
            .map_err(|err| LinkedErr::from(err, self))?
        {
            Some(entity)
        } else {
            let last_name: String = self.get_last_name();
            scx.lookup_fn_by_inps(&last_name, &tys.iter().collect::<Vec<&Ty>>(), &self.uuid)
        }) else {
            return Err(LinkedErr::from(E::FnNotFound(name), self));
        };
        let fn_args = entity.args_tys();
        let mut vl_tys = tys.iter();
        let mut repeated = false;
        for fn_arg in fn_args.iter() {
            if repeated {
                return Err(LinkedErr::from(E::MultipleRepeatedFnArgs, self));
            }
            match fn_arg {
                Ty::Determined(..) | Ty::OneOf(..) | Ty::Variants(..) | Ty::Optional(..) => {
                    let vl_ty = vl_tys.next().ok_or(LinkedErr::from(
                        E::FnArgsNumberDismatch(name.clone(), fn_args.len(), tys.len()),
                        self,
                    ))?;
                    let vl_ty = vl_ty.determined().ok_or(LinkedErr::from(
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
                        return Err(LinkedErr::from(
                            E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                            self,
                        ));
                    }
                }
                Ty::Repeated(arg_ty) => {
                    repeated = true;
                    for vl_ty in vl_tys.by_ref() {
                        let vl_ty = vl_ty.determined().ok_or(LinkedErr::from(
                            E::FailInferDeterminedType(vl_ty.clone()),
                            self,
                        ))?;
                        if !arg_ty.compatible(vl_ty) {
                            return Err(LinkedErr::from(
                                E::DismatchTypes(format!("{fn_arg} and {vl_ty}")),
                                self,
                            ));
                        }
                    }
                }
                Ty::Undefined | Ty::Indeterminate => {
                    return Err(LinkedErr::from(E::InvalidFnArg, self));
                }
            }
        }
        if vl_tys.next().is_some() {
            return Err(LinkedErr::from(
                E::FnArgsNumberDismatch(name, fn_args.len(), tys.len()),
                self,
            ));
        }
        Ok(())
    }
}

impl SemanticTokensGetter for FunctionCall {
    fn get_semantic_tokens(&self, _stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken> {
        let (Some((_, left)), Some((_, right))) = (self.reference.first(), self.reference.last())
        else {
            return Vec::new();
        };
        let mut tokens = vec![
            LinkedSemanticToken::between_tokens(left, right, SemanticToken::Function),
            LinkedSemanticToken::from_token(&self.open, SemanticToken::Delimiter),
            LinkedSemanticToken::from_token(&self.close, SemanticToken::Delimiter),
        ];
        tokens.extend(
            self.args
                .iter()
                .flat_map(|n| n.get_semantic_tokens(SemanticTokenContext::FunctionCall)),
        );
        tokens
    }
}
