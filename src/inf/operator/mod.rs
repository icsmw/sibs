mod error;
use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{any::AnyValue, context::Context},
};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a>>;
pub type OperatorResult = Result<Option<AnyValue>, E>;

pub trait Operator {
    fn token(&self) -> usize;
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            cx.sources.set_map_cursor(&self.token())?;
            let result = self.perform(owner, components, args, cx).await;
            match result.as_ref() {
                Ok(value) => {
                    cx.sources.set_trace_value(&self.token(), value);
                }
                Err(err) => {
                    cx.sources.report_error(&self.token(), err)?;
                }
            }
            result
        })
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [String],
        _cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported) })
    }
}
