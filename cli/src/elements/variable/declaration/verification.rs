use crate::{
    elements::{Element, VariableDeclaration},
    error::LinkedErr,
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for VariableDeclaration {
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
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            let Element::VariableName(el, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self));
            };
            cx.variables
                .set(
                    &owner.as_component()?.uuid,
                    el.get_name(),
                    self.declaration
                        .expected(owner, components, prev, cx)
                        .await?,
                )
                .await
                .map_err(|e| LinkedErr::new(e, Some(self.token)))?;
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
            self.declaration
                .try_expected(owner, components, prev, cx)
                .await
        })
    }
}
