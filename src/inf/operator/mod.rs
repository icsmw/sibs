mod error;
use crate::{
    inf::{any::AnyValue, context::Context},
    reader::entry::Component,
};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a>>;
pub type OperatorResult = Result<Option<AnyValue>, E>;

pub trait Operator {
    fn process<'a>(
        &'a self,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported) })
    }
}
