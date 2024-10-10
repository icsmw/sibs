use crate::{
    elements::{Element, TokenGetter, VariableName},
    error::LinkedErr,
    inf::{
        Context, ExpectedResult, LinkingResult, PrevValueExpectation, TryExpectedValueType,
        VerificationResult,
    },
};

impl TryExpectedValueType for VariableName {
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
        owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            cx.variables
                .get(&owner.as_component()?.uuid, &self.name)
                .await
                .map_err(|e| LinkedErr::new(e, Some(self.token())))
        })
    }
}
