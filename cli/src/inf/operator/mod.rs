mod error;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{AnyValue, Context, Scope},
};
pub use error::E;
use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub type OperatorPinnedResult<'a> = Pin<Box<dyn Future<Output = OperatorResult> + 'a + Send>>;
pub type OperatorResult = Result<Option<AnyValue>, LinkedErr<E>>;

pub trait Operator {
    fn token(&self) -> usize;
    // fn el_target(&self) -> ElTarget;
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> OperatorPinnedResult
    where
        Self: Sync,
    {
        Box::pin(async move {
            if cx.is_aborting() {
                cx.journal.warn("runner", "skipping, because aborting");
                return Ok(None);
            }
            cx.atlas.set_map_position(self.token()).await?;
            let result = self
                .perform(owner, components, args, cx.clone(), sc, token)
                .await;
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
        _token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async { Err(E::NotSupported.unlinked()) })
    }
}
