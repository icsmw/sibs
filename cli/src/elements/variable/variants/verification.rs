use crate::{
    elements::{Element, VariableVariants},
    inf::{
        Context, ExpectedResult, LinkingResult, PrevValueExpectation, TryExpectedValueType,
        ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for VariableVariants {
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
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            // TODO: needs implementation
            Ok(ValueRef::String)
        })
    }
}
