mod error;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{Context, Scope, Value},
};
pub use error::E;
use std::{future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub type ExecutePinnedResult<'a> = Pin<Box<dyn Future<Output = ExecuteResult> + 'a + Send>>;
pub type ExecuteResult = Result<Option<Value>, LinkedErr<E>>;

pub trait TokenGetter {
    fn token(&self) -> usize;
}
pub trait TryExecute {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [Value],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async { Err(E::NotSupported.unlinked()) })
    }
}

pub trait Execute {
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [Value],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult
    where
        Self: TryExecute + TokenGetter + Sync,
    {
        Box::pin(async move {
            if cx.is_aborting() {
                cx.journal.warn("runner", "skipping, because aborting");
                return Ok(None);
            }
            cx.atlas.set_map_position(self.token()).await?;
            let result = self
                .try_execute(owner, components, args, cx.clone(), sc, token)
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
}
