use std::fmt::Debug;

use asttree::LinkedNode;

use crate::*;

pub type UserFnExecutor =
    Box<dyn Fn(Runtime) -> RtPinnedResult<'static, LinkedErr<E>> + Send + Sync>;

pub enum FnBody {
    Node(LinkedNode),
    Executor(Metadata, UserFnExecutor),
}

impl Debug for FnBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(n) => format!("FnBody::Node({n:?})"),
                Self::Executor(..) => "FnBody::Executor(..)".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct UserFnEntity {
    pub uuid: Uuid,
    pub name: String,
    pub args: Vec<UserFnArgDeclaration>,
    pub result: Ty,
    pub body: FnBody,
}

impl UserFnEntity {
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
    ) -> Result<RtValue, LinkedErr<E>> {
        let FnBody::Executor(md, exec) = &self.body else {
            return Err(LinkedErr::unlinked(E::NotInitedFunction(
                self.name.to_owned(),
            )));
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
                    E::FnArgumentTypeDismatch(decl.ty.to_string()),
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
