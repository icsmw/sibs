use crate::{
    elements::{Element, Return},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Return {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            if let Some(el) = self.output.as_ref() {
                el.verification(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }
    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            if let Some(el) = self.output.as_ref() {
                el.linking(owner, components, prev, cx).await?;
            }
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
        Box::pin(async move {
            Ok(if let Some(el) = self.output.as_ref() {
                el.expected(owner, components, prev, cx).await?
            } else {
                ValueRef::Empty
            })
        })
    }
}
