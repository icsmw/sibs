use crate::*;

pub struct EmbeddedFnArg {
    pub value: RtValue,
    pub link: SrcLink,
}

pub type ExecutorEmbeddedFn =
    fn(Vec<EmbeddedFnArg>, Runtime) -> RtPinnedResult<'static, LinkedErr<E>>;

#[derive(Debug)]
pub struct EmbeddedFnEntity {
    pub uuid: Uuid,
    pub fullname: String,
    pub name: String,
    pub args: Vec<DataType>,
    pub result: DataType,
    pub exec: ExecutorEmbeddedFn,
}

impl EmbeddedFnEntity {
    pub async fn execute(&self, rt: Runtime, args: Vec<RtValue>) -> Result<RtValue, LinkedErr<E>> {
        Ok(RtValue::Void)
    }
}
