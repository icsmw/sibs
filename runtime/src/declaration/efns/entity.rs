use crate::*;
use std::{future::Future, pin::Pin};

pub type EmbeddedFnPinnedResult<'a, E> =
    Pin<Box<dyn Future<Output = EmbeddedFnResult<E>> + 'a + Send>>;
pub type EmbeddedFnResult<E> = Result<RtValue, E>;

#[derive(Debug)]
pub struct EmbeddedFnEntity {
    pub uuid: Uuid,
    pub name: String,
    pub args: Vec<DataType>,
    pub result: DataType,
}
