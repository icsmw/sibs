use crate::{
    elements::{Compute, Element},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Compute {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.left.verification(owner, components, prev, cx).await?;
            self.right.verification(owner, components, prev, cx).await?;
            let left = self.left.expected(owner, components, prev, cx).await?;
            let right = self.right.expected(owner, components, prev, cx).await?;
            if !left.is_numeric() || !right.is_numeric() {
                Err(operator::E::ArithmeticWrongType.linked(&self.token))
            } else {
                Ok(())
            }
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
            self.left.linking(owner, components, prev, cx).await?;
            self.right.linking(owner, components, prev, cx).await
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::isize) })
    }
}
