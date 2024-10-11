use crate::{
    elements::{Each, Element},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for Each {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            self.input.linking(owner, components, prev, cx).await?;
            self.block.linking(owner, components, prev, cx).await?;
            self.variable.linking(owner, components, prev, cx).await?;
            Ok(())
        })
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
