use std::fmt::Debug;

use crate::*;

pub type TaskExecutor = Box<dyn Fn(Runtime) -> RtPinnedResult<'static, LinkedErr<E>> + Send + Sync>;

pub enum TaskBody {
    Node(Task),
    Executor(SrcLink, TaskExecutor),
}

impl Debug for TaskBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Node(n) => format!("TaskBody::Node({n:?})"),
                Self::Executor(..) => "TaskBody::Executor(..)".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
pub struct TaskEntity {
    pub uuid: Uuid,
    pub name: String,
    pub master: MasterComponent,
    pub args: Vec<TaskArgDeclaration>,
    pub result: Ty,
    pub body: TaskBody,
}

impl TaskEntity {
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
        if let Some(keyword) = self
            .fullname()
            .split(":")
            .find(|p| Keyword::try_from(*p).is_ok())
        {
            Err(E::FnUsesKeyword(self.fullname(), keyword.to_owned()))
        } else {
            Ok(())
        }
    }
    pub async fn execute(
        &self,
        rt: Runtime,
        args: Vec<FnArgValue>,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        let TaskBody::Executor(link, exec) = &self.body else {
            return Err(LinkedErr::by_link(
                E::NotInitedTask(self.fullname()),
                caller.into(),
            ));
        };
        if let Err(err) = rt.scopes.enter(&self.uuid).await {
            return Err(LinkedErr::by_link(err, link.into()));
        }
        let mut err = None;
        for (n, arg_vl) in args.into_iter().enumerate() {
            let Some(decl) = self.args.get(n) else {
                err = Some(LinkedErr::by_link(E::InvalidTaskArgument, link.into()));
                break;
            };
            let Some(vl_ty) = arg_vl.value.as_ty() else {
                err = Some(LinkedErr::by_link(
                    E::InvalidTaskArgumentType,
                    (&arg_vl.link).into(),
                ));
                break;
            };
            if !decl.ty.compatible(&vl_ty) {
                err = Some(LinkedErr::by_link(
                    E::TaskArgumentTypeDismatch(decl.ty.to_string()),
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
                return Err(LinkedErr::by_link(err, link.into()));
            }
            return Err(err);
        }
        let result = exec(rt.clone()).await;
        if let Err(err) = rt.scopes.leave().await {
            return Err(LinkedErr::by_link(err, link.into()));
        }
        result
    }

    pub fn args_tys(&self) -> Vec<&Ty> {
        self.args.iter().map(|arg| &arg.ty).collect::<Vec<&Ty>>()
    }

    pub fn fullname(&self) -> String {
        format!("{}:{}", self.master.name, self.name)
    }
}
