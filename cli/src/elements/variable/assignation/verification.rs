use crate::{
    elements::{Element, VariableAssignation},
    error::LinkedErr,
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for VariableAssignation {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.variable
                .verification(owner, components, prev, cx)
                .await?;
            self.assignation
                .verification(owner, components, prev, cx)
                .await
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
            let Element::VariableName(el, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self));
            };
            self.variable.linking(owner, components, prev, cx).await?;
            self.assignation
                .linking(owner, components, prev, cx)
                .await?;
            cx.variables
                .set(
                    &owner.as_component()?.uuid,
                    el.get_name(),
                    self.assignation
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
        Box::pin(async move { self.assignation.expected(owner, components, prev, cx).await })
    }
}
