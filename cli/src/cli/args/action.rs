use crate::{cli::error::E, elements::Element, inf::AnyValue};
use std::{fmt::Debug, future::Future, pin::Pin};

pub type ActionPinnedResult<'a> = Pin<Box<dyn Future<Output = ActionResult> + 'a>>;
pub type ActionResult = Result<AnyValue, E>;

pub trait Action: Debug {
    fn action<'a>(&'a self, _components: &'a [Element]) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::empty()) })
    }
    fn no_context(&self) -> bool {
        false
    }
    fn key(&self) -> String;
}
