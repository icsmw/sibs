use crate::{
    elements::{Element, If},
    inf::{
        operator, Context, ExpectedResult, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for If {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for thr in self.threads.iter() {
                thr.try_verification(owner, components, prev, cx).await?;
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
            for thr in self.threads.iter() {
                thr.try_linking(owner, components, prev, cx).await?;
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
            let mut refs: Option<ValueRef> = None;
            for value_ref in self.threads.iter() {
                if let Some(prev_value) = refs.as_ref() {
                    if prev_value != &value_ref.try_expected(owner, components, prev, cx).await? {
                        return Err(operator::E::ReturnsDifferentTypes.by(self));
                    }
                } else {
                    refs = Some(value_ref.try_expected(owner, components, prev, cx).await?);
                }
            }
            Ok(refs.unwrap_or(ValueRef::Empty))
        })
    }
}
