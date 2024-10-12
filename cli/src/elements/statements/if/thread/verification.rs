use crate::{
    elements::{Element, IfThread},
    inf::{
        Context, ExpectedResult, ExpectedValueType, LinkingResult, PrevValueExpectation,
        TryExpectedValueType, VerificationResult,
    },
};

impl TryExpectedValueType for IfThread {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            match self {
                Self::If(sub, bl) => {
                    sub.verification(owner, components, prev, cx).await?;
                    bl.verification(owner, components, prev, cx).await?;
                }
                Self::Else(bl) => {
                    bl.verification(owner, components, prev, cx).await?;
                }
            };
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
            match self {
                Self::If(sub, bl) => {
                    sub.linking(owner, components, prev, cx).await?;
                    bl.linking(owner, components, prev, cx).await?;
                }
                Self::Else(bl) => {
                    bl.linking(owner, components, prev, cx).await?;
                }
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
            match self {
                Self::If(_, block) => block.expected(owner, components, prev, cx).await,
                Self::Else(block) => block.expected(owner, components, prev, cx).await,
            }
        })
    }
}
