use std::fmt::Debug;

use crate::*;

pub enum ClosureFnBody {
    Node(Closure),
    Executor(SrcLink, UserFnExecutor),
    Declaration,
}

impl Debug for ClosureFnBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(n) => format!("ClosureFnBody::Node({n:?})"),
                Self::Executor(..) => "ClosureFnBody::Executor(..)".to_owned(),
                Self::Declaration => "ClosureFnBody::Declaration".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct ClosureFnEntity {
    pub uuid: Uuid,
    pub args: Vec<UserFnArgDeclaration>,
    pub result: Ty,
    pub body: ClosureFnBody,
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
        cx: Context,
        args: Vec<FnArgValue>,
        _fns: &Fns,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        let ClosureFnBody::Executor(link, exec) = &self.body else {
            return Err(LinkedErr::by_link(
                E::NotInitedClosure(self.uuid),
                caller.into(),
            ));
        };
        if let Err(err) = cx.location().enter(&self.uuid).await {
            return Err(LinkedErr::by_link(err, link.into()));
        }
        let mut err = None;
        for (n, arg_vl) in args.into_iter().enumerate() {
            let Some(decl) = self.args.get(n) else {
                err = Some(LinkedErr::by_link(E::InvalidFnArgument, link.into()));
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
            if let Err(e) = cx.values().insert(&decl.ident, arg_vl.value).await {
                err = Some(LinkedErr::by_link(e, (&arg_vl.link).into()));
                break;
            }
        }
        if let Some(err) = err.take() {
            if let Err(err) = cx.location().leave().await {
                return Err(LinkedErr::by_link(err, link.into()));
            }
            return Err(err);
        }
        let result = exec(rt.clone(), cx.clone()).await;
        if let Err(err) = cx.location().leave().await {
            return Err(LinkedErr::by_link(err, link.into()));
        }
        result
    }
}
