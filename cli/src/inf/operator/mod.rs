mod error;
pub mod variables;

use crate::{
    elements::{Component, Element, Metadata},
    error::LinkedErr,
    inf::{Context, Scope, Value, ValueRef},
};
pub use error::E;
use std::{fmt::Debug, future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;
pub use variables::*;

pub type ExecutePinnedResult<'a> = Pin<Box<dyn Future<Output = ExecuteResult> + 'a + Send>>;
pub type ExecuteResult = Result<Option<Value>, LinkedErr<E>>;
pub type LinkingResult<'a> = Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type VerificationResult<'a> =
    Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type ExpectedResult<'a> =
    Pin<Box<dyn Future<Output = Result<ValueRef, LinkedErr<E>>> + 'a + Send>>;

pub trait TokenGetter {
    fn token(&self) -> usize;
}

pub trait ExpectedValueType {
    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult;

    fn varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> VerificationResult;

    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> ExpectedResult;
}

pub trait TryExecute {
    #[allow(clippy::too_many_arguments)]
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Element>,
        _components: &'a [Element],
        _args: &'a [Value],
        _prev: &'a Option<Value>,
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async { Err(E::NotSupported.unlinked()) })
    }
}

pub trait Execute {
    #[allow(clippy::too_many_arguments)]
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult
    where
        Self: TryExecute + TokenGetter + ExpectedValueType + Debug + Sync,
    {
        Box::pin(async move {
            if cx.is_aborting() {
                cx.journal.warn("runner", "skipping, because aborting");
                return Ok(None);
            }
            cx.atlas.set_map_position(self.token()).await?;
            let result = self
                .try_execute(owner, components, args, prev, cx.clone(), sc, token)
                .await;
            match result.as_ref() {
                Ok(value) => {
                    cx.atlas.add_footprint(self.token(), value).await?;
                    if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                        //
                    }
                }
                Err(err) => {
                    cx.atlas.report_err(&err.link_if(&self.token())).await?;
                }
            }
            result
        })
    }
    fn get_metadata(&self) -> Result<&Metadata, LinkedErr<E>>
    where
        Self: TokenGetter + Debug + Sync,
    {
        Err(E::AttemptToGetMetadataOutOfElement(format!("{self:?}")).linked(&self.token()))
    }
}
