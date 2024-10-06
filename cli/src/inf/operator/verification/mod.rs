use crate::{
    elements::{Element, TokenGetter},
    error::LinkedErr,
    inf::{
        operator::{Execute, E},
        Context, PrevValueExpectation, ValueRef,
    },
};
use std::{fmt::Debug, future::Future, pin::Pin};

pub type LinkingResult<'a> = Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type VerificationResult<'a> =
    Pin<Box<dyn Future<Output = Result<(), LinkedErr<E>>> + 'a + Send>>;
pub type ExpectedResult<'a> =
    Pin<Box<dyn Future<Output = Result<ValueRef, LinkedErr<E>>> + 'a + Send>>;

pub trait TryExpectedValueType {
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a>;

    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a>;

    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a>;
}

pub trait ExpectedValueType {
    fn linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a>
    where
        Self: TryExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            if let Some(ppm) = self.get_metadata()?.ppm.as_ref() {
                ppm.linking(
                    owner,
                    components,
                    &Some(PrevValueExpectation {
                        token: self.token(),
                        value: self.try_expected(owner, components, prev, cx).await?,
                    }),
                    cx,
                )
                .await?;
            }
            self.try_linking(owner, components, prev, cx).await
        })
    }

    fn verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a>
    where
        Self: TryExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            if let Some(el) = self.get_metadata()?.ppm.as_ref() {
                el.verification(
                    owner,
                    components,
                    &Some(PrevValueExpectation {
                        token: self.token(),
                        value: self.try_expected(owner, components, prev, cx).await?,
                    }),
                    cx,
                )
                .await?;
            }
            self.try_verification(owner, components, prev, cx).await
        })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a>
    where
        Self: TryExpectedValueType + ExpectedValueType + Execute + TokenGetter + Debug + Sync,
    {
        Box::pin(async move {
            let value = self.try_expected(owner, components, prev, cx).await?;
            Ok(if let Some(el) = self.get_metadata()?.ppm.as_ref() {
                el.expected(
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
