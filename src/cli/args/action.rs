use crate::{
    cli::error::E,
    elements::Component,
    inf::{context::Context, AnyValue},
};
use std::{fmt::Debug, future::Future, pin::Pin};

pub type ActionPinnedResult<'a> = Pin<Box<dyn Future<Output = ActionResult> + 'a>>;
pub type ActionResult = Result<AnyValue, E>;

pub trait Action: Debug {
    fn action<'a>(
        &'a self,
        _components: &'a [Component],
        _context: &'a mut Context,
    ) -> ActionPinnedResult {
        Box::pin(async move { Ok(AnyValue::new(())) })
    }
    fn no_context(&self) -> bool {
        false
    }
    fn key(&self) -> String;
}
