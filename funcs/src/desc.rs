use crate::*;

pub type FnBody = fn(Vec<FnArg>) -> RtPinnedResult<LinkedErr<E>>;

pub struct FnDesc {
    pub body: FnBody,
    pub args: Vec<DataType>,
    pub result: DataType,
}
