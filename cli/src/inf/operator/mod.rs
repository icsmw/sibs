mod error;

use crate::{
    elements::{Element, Metadata},
    error::LinkedErr,
    inf::{Context, PrevValue, PrevValueExpectation, Scope, Value, ValueRef},
};
pub use error::E;
use std::{fmt::Debug, future::Future, pin::Pin};
use tokio_util::sync::CancellationToken;

pub type ExecutePinnedResult<'a> = Pin<Box<dyn Future<Output = ExecuteResult> + 'a + Send>>;
pub type ExecuteResult = Result<Value, LinkedErr<E>>;
pub type LinkingResult<'a> = Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type VerificationResult<'a> =
    Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type ExpectedResult<'a> =
    Pin<Box<dyn Future<Output = Result<ValueRef, LinkedErr<E>>> + 'a + Send>>;

pub trait TokenGetter {
    fn token(&self) -> usize;
}

pub trait TryExpectedValueType {
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult;

    fn try_varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult;

    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult;
}

pub trait ExpectedValueType {
    fn linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult
    where
        Self: TryExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                let value = self.try_expected(owner, components, prev, cx).await?;
                ppm.linking(
                    owner,
                    components,
                    &Some(PrevValueExpectation {
                        token: self.token(),
                        value,
                    }),
                    cx,
                )
                .await?;
            }
            self.try_linking(owner, components, prev, cx).await
        })
    }

    fn varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult
    where
        Self: TryExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                let value = self.try_expected(owner, components, prev, cx).await?;
                ppm.varification(
                    owner,
                    components,
                    &Some(PrevValueExpectation {
                        token: self.token(),
                        value,
                    }),
                    cx,
                )
                .await?;
            }
            self.try_varification(owner, components, prev, cx).await
        })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult
    where
        Self: TryExpectedValueType + ExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            let value = self.try_expected(owner, components, prev, cx).await?;
            Ok(if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                ppm.expected(
                    owner,
                    components,
                    &Some(PrevValueExpectation {
                        token: self.token(),
                        value,
                    }),
                    cx,
                )
                .await?
            } else {
                value
            })
        })
    }
}

pub trait TryExecute {
    #[allow(clippy::too_many_arguments)]
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult;
}

pub trait Execute {
    #[allow(clippy::too_many_arguments)]
    fn execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult
    where
        Self: TryExecute + TokenGetter + TryExpectedValueType + Debug + Sync,
    {
        Box::pin(async move {
            if cx.is_aborting() {
                cx.journal.warn("runner", "skipping, because aborting");
                return Ok(Value::empty());
            }
            cx.atlas.set_map_position(self.token()).await?;
            let result = self
                .try_execute(
                    owner,
                    components,
                    args,
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await;
            match result {
                Ok(value) => {
                    cx.atlas.add_footprint(self.token(), &value).await?;
                    Ok(if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                        ppm.execute(
                            owner,
                            components,
                            args,
                            &Some(PrevValue {
                                value,
                                token: self.token(),
                            }),
                            cx.clone(),
                            sc.clone(),
                            token.clone(),
                        )
                        .await?
                    } else {
                        value
                    })
                }
                Err(err) => {
                    cx.atlas.report_err(&err.link_if(&self.token())).await?;
                    Err(err)
                }
            }
        })
    }
    fn get_metadata(&self) -> Result<&Metadata, LinkedErr<E>>
    where
        Self: TokenGetter + Debug + Sync,
    {
        Err(E::AttemptToGetMetadataOutOfElement(format!("{self:?}")).linked(&self.token()))
    }
}
