use crate::{
    elements::{Element, For},
    error::LinkedErr,
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for For {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.index.verification(owner, components, prev, cx).await?;
            self.target
                .verification(owner, components, prev, cx)
                .await?;
            self.block.verification(owner, components, prev, cx).await
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
            if let Element::VariableName(el, _) = &*self.index {
                cx.variables
                    .set(&owner.as_component()?.uuid, el.get_name(), ValueRef::usize)
                    .await
                    .map_err(|e| LinkedErr::new(e, Some(self.token)))?;
            }
            self.index.linking(owner, components, prev, cx).await?;
            self.target.linking(owner, components, prev, cx).await?;
            self.block.linking(owner, components, prev, cx).await
        })
    }

    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}
