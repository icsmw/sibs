use crate::*;

pub type ExecutorEmbeddedFn = fn(Vec<FnArgValue>, Runtime) -> RtPinnedResult<'static, LinkedErr<E>>;

#[derive(Debug)]
pub struct EmbeddedFnEntity {
    pub uuid: Uuid,
    pub fullname: String,
    pub name: String,
    pub args: Vec<Ty>,
    pub result: DeterminatedTy,
    pub exec: ExecutorEmbeddedFn,
}

impl EmbeddedFnEntity {
    pub async fn execute(
        &self,
        rt: Runtime,
        args: Vec<FnArgValue>,
    ) -> Result<RtValue, LinkedErr<E>> {
        (self.exec)(args, rt).await
    }
}
