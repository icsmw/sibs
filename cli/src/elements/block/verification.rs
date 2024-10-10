use crate::{
    elements::{Block, Element},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Block {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.elements.iter() {
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
            for el in self.elements.iter() {
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
            let Some(el) = self.elements.last() else {
                return Ok(ValueRef::Empty);
            };
            el.expected(owner, components, prev, cx).await
        })
    }
}
