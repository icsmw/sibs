use crate::*;

pub type ExecutorEmbeddedFn = fn(
    Vec<FnArgValue>,
    Runtime,
    Context,
    caller: SrcLink,
) -> RtPinnedResult<'static, LinkedErr<E>>;

#[derive(Debug)]
pub struct EmbeddedFnEntity {
    pub uuid: Uuid,
    pub fullname: String,
    pub name: String,
    pub args: Vec<Ty>,
    pub result: DeterminedTy,
    pub exec: ExecutorEmbeddedFn,
}

impl EmbeddedFnEntity {
    pub fn verify(&self) -> Result<(), E> {
        if self.args.iter().any(|arg| matches!(arg, Ty::Repeated(..))) {
            if self
                .args
                .iter()
                .filter(|arg| matches!(arg, Ty::Repeated(..)))
                .count()
                > 1
            {
                return Err(E::MultipleRepeatedFnArgsDeclared);
            }
            if let Some(last) = self.args.last() {
                if !matches!(last, Ty::Repeated(..)) {
                    return Err(E::NotLastRepeatedFnArg);
                }
            }
        }
        if let Some(keyword) = self
            .fullname
            .split("::")
            .find(|p| Keyword::try_from(*p).is_ok())
        {
            Err(E::FnUsesKeyword(
                self.fullname.to_owned(),
                keyword.to_owned(),
            ))
        } else {
            Ok(())
        }
    }
    pub async fn execute(
        &self,
        rt: Runtime,
        cx: Context,
        args: Vec<FnArgValue>,
        _fns: &Fns,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        (self.exec)(args, rt, cx, caller.clone()).await
    }
    pub fn compatible(&self, incomes: &[&Ty]) -> bool {
        FnEntity::args_compatible(&self.args.iter().collect::<Vec<&Ty>>(), incomes)
    }
}
