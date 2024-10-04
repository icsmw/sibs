use crate::{cli::error::E, elements::Element, inf::Value};
use std::{fmt::Debug, future::Future, pin::Pin};

pub type ActionPinnedResult<'a> = Pin<Box<dyn Future<Output = ActionResult> + 'a>>;
pub type ActionResult = Result<Value, E>;

pub trait Action: Debug {
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult<'a> {
        Box::pin(async move { Ok(Value::empty()) })
    }
    fn no_context(&self) -> bool {
        false
    }
    fn key(&self) -> String;
}
