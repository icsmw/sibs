use crate::{
    elements::{Element, Task},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Task {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.dependencies.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            for el in self.declarations.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
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
            for el in self.dependencies.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            for el in self.declarations.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            self.block.linking(owner, components, prev, cx).await
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
            let mut args: Vec<ValueRef> = Vec::new();
            for el in self.declarations.iter() {
                args.push(el.expected(owner, components, prev, cx).await?);
            }
            Ok(ValueRef::Task(
                args,
                Box::new(self.block.expected(owner, components, prev, cx).await?),
            ))
        })
    }
}
