use crate::{
    elements::{Element, First},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for First {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { self.block.verification(owner, components, prev, cx).await })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.block.linking(owner, components, prev, cx).await })
    }

    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { self.block.expected(owner, components, prev, cx).await })
    }
}
