use crate::{
    elements::{Accessor, Element},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Accessor {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { self.index.verification(owner, components, prev, cx).await })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.index.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            Ok(match &prev_value.value {
                ValueRef::String => ValueRef::String,
                ValueRef::Vec(ty) => *ty.clone(),
                el => {
                    return Err(
                        operator::E::NotSupportedTypeByAccessor(el.to_owned()).linked(&self.token)
                    )?;
                }
            })
        })
    }
}
