use std::fmt::Debug;

use crate::*;

#[derive(Debug)]
pub struct ClosureFnEntity {
    pub uuid: Uuid,
    pub args: Vec<UserFnArgDeclaration>,
    pub result: Ty,
    pub body: FnBody,
}

impl ClosureFnEntity {
    pub fn verify(&self) -> Result<(), E> {
        if self
            .args
            .iter()
            .any(|arg| matches!(arg.ty, Ty::Repeated(..)))
        {
            if self
                .args
                .iter()
                .filter(|arg| matches!(arg.ty, Ty::Repeated(..)))
                .count()
                > 1
            {
                return Err(E::MultipleRepeatedFnArgsDeclared);
            }
            if let Some(last) = self.args.last() {
                if !matches!(last.ty, Ty::Repeated(..)) {
                    return Err(E::NotLastRepeatedFnArg);
                }
            }
        }
        Ok(())
    }
    pub async fn execute(
        &self,
        rt: Runtime,
        args: Vec<FnArgValue>,
        _fns: &Fns,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        let FnBody::Executor(md, exec) = &self.body else {
            return Err(LinkedErr::by_link(
                E::NotInitedClosure(self.uuid),
                caller.into(),
            ));
        };
        if let Err(err) = rt.scopes.enter(&self.uuid).await {
            return Err(LinkedErr::by_link(err, (&md.link).into()));
        }
        let mut err = None;
        for (n, arg_vl) in args.into_iter().enumerate() {
            let Some(decl) = self.args.get(n) else {
                err = Some(LinkedErr::by_link(E::InvalidFnArgument, (&md.link).into()));
                break;
            };
            let Some(vl_ty) = arg_vl.value.as_ty() else {
                err = Some(LinkedErr::by_link(
                    E::InvalidFnArgumentType,
                    (&arg_vl.link).into(),
                ));
                break;
            };
            if !decl.ty.compatible(&vl_ty) {
                err = Some(LinkedErr::by_link(
                    E::FnArgumentTypeDismatch(format!("{} vs {vl_ty}", decl.ty)),
                    (&arg_vl.link).into(),
                ));
                break;
            }
            if let Err(e) = rt.scopes.insert(&decl.ident, arg_vl.value).await {
                err = Some(LinkedErr::by_link(e, (&arg_vl.link).into()));
                break;
            }
        }
        if let Some(err) = err.take() {
            if let Err(err) = rt.scopes.leave().await {
                return Err(LinkedErr::by_link(err, (&md.link).into()));
            }
            return Err(err);
        }
        let result = exec(rt.clone()).await;
        if let Err(err) = rt.scopes.leave().await {
            return Err(LinkedErr::by_link(err, (&md.link).into()));
        }
        result
    }
}
