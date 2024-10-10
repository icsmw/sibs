use crate::{
    elements::{Element, Reference},
    inf::{
        operator, Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Reference {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.inputs.iter() {
                el.verification(owner, components, prev, cx).await?
            }
            let task_el = self.get_linked_task(owner, components)?;
            let task_ref = task_el.as_task()?;
            let ValueRef::Task(args, _) = task_el.expected(owner, components, prev, cx).await?
            else {
                return Err(operator::E::InvalidValueRef(format!(
                    "task \"{}\" has invalid expected output",
                    task_ref.get_name()
                ))
                .by(self));
            };
            if args.len() != self.inputs.len() {
                return Err(operator::E::InvalidValueRef(format!(
                    "arguments count for task \"{}\" dismatch with reference inputs",
                    task_ref.get_name()
                ))
                .by(self));
            }
            for (i, el) in self.inputs.iter().enumerate() {
                el.verification(owner, components, prev, cx).await?;
                let left = el.expected(owner, components, prev, cx).await?;
                let right = &args[i];
                if !left.is_compatible(right) {
                    return Err(operator::E::DismatchTypes(left, right.clone()).by(self));
                }
            }
            Ok(())
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
            for el in self.inputs.iter() {
                el.linking(owner, components, prev, cx).await?
            }
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
            self.get_linked_task(owner, components)?
                .expected(owner, components, prev, cx)
                .await
        })
    }
}
