use crate::{
    elements::{Comparing, Element},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Comparing {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            let left = self.left.expected(owner, components, prev, cx).await?;
            let right = self.right.expected(owner, components, prev, cx).await?;
            if !left.is_compatible(&right) {
                Err(operator::E::DismatchTypes(left, right).by(self))
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
            self.right.linking(owner, components, prev, cx).await?;
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}
