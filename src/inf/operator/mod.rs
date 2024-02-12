mod error;
use crate::{
    entry::Component,
    inf::{any::AnyValue, context::Context},
};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a>>;
pub type OperatorResult = Result<Option<AnyValue>, E>;

pub trait Operator {
    fn process<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [String],
        _cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported) })
    }
}
