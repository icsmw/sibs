use crate::{
    elements::{typed::Types, Element, VariableType},
    inf::{
        Context, ExpectedResult, LinkingResult, PrevValueExpectation, TryExpectedValueType,
        ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for VariableType {
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
            Ok(match self.var_type {
                Types::String => ValueRef::String,
                Types::Bool => ValueRef::bool,
                Types::Number => ValueRef::isize,
            })
        })
    }
}
