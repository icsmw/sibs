use crate::{
    elements::{Element, Join},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Join {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.elements
                .verification(owner, components, prev, cx)
                .await
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.elements.linking(owner, components, prev, cx).await })
    }

    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Vec(Box::new(ValueRef::SpawnStatus))) })
    }
}
