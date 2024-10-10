use crate::{
    elements::{Closure, Element},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Closure {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            // let parent = self
            //     .owner
            //     .as_ref()
            //     .ok_or(operator::E::ClosureIsNotBoundWithOwner)?;
            // for el in self.args.iter() {
            //     el.verification(owner, components, prev, cx).await?;
            // }
            // self.block.verification(owner, components, prev, cx).await?;
            // let desc = cx
            //     .get_func_desc(parent, prev.as_ref().map(|v| v.value.clone()).clone())
            //     .await?;
            // todo!("Not implemented");
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
            for el in self.args.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            self.block.linking(owner, components, prev, cx).await?;
            cx.closures.set(self.uuid, self.clone()).await?;
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Closure) })
    }
}
