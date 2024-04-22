mod error;
use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{AnyValue, Context, Scope},
};
pub use error::E;
use std::{future::Future, pin::Pin};

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a + Send>>;
pub type OperatorResult = Result<Option<AnyValue>, LinkedErr<E>>;

pub trait Operator {
    fn token(&self) -> usize;
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
    ) -> OperatorPinnedResult
    where
        Self: Sync,
    {
        Box::pin(async move {
            cx.atlas.set_map_position(self.token()).await?;
            let result = self.perform(owner, components, args, cx.clone(), sc).await;
            match result.as_ref() {
                Ok(value) => {
                    cx.atlas.add_footprint(self.token(), value).await?;
                }
                Err(err) => {
                    cx.atlas.report_err(&err.link_if(&self.token())).await?;
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
        _cx: Context,
        _sc: Scope,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported.unlinked()) })
    }
}
