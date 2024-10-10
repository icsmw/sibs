use crate::{
    elements::{Element, Range, TokenGetter},
    inf::{
        operator::E, Context, ExpectedResult, ExpectedValueType, LinkingResult,
        PrevValueExpectation, TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Range {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            if self
                .from
                .expected(owner, components, prev, cx)
                .await?
                .is_compatible(&ValueRef::Numeric)
            {
                return Err(E::ExpectedNumericValue.linked(&self.from.token()));
            }
            if self
                .to
                .expected(owner, components, prev, cx)
                .await?
                .is_compatible(&ValueRef::Numeric)
            {
                return Err(E::ExpectedNumericValue.linked(&self.to.token()));
            }
            self.from.verification(owner, components, prev, cx).await?;
            self.to.verification(owner, components, prev, cx).await
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
            self.from.linking(owner, components, prev, cx).await?;
            self.to.linking(owner, components, prev, cx).await
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Vec(Box::new(ValueRef::usize))) })
    }
}
