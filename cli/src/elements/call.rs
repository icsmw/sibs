use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Call {
    pub func: Box<Element>,
    pub token: usize,
}

impl TryDissect<Call> for Call {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Call>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Call);
        Ok(if reader.move_to().char(&[&chars::DOT]).is_some() {
            if let Some(chars::DOT) = reader.next().char() {
                None
            } else {
                let Some(el) = Element::include(reader, &[ElementRef::Function])? else {
                    return Err(E::NoCallFunction.linked(&close(reader)));
                };
                Some(Call {
                    func: Box::new(el),
                    token: close(reader),
                })
            }
        } else {
            None
        })
    }
}

impl Dissect<Call, Call> for Call {}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".{}", self.func)
    }
}

impl Formation for Call {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!(".{}", self.func)
    }
}

impl TokenGetter for Call {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Call {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { self.func.verification(owner, components, prev, cx).await })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.func.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { self.func.expected(owner, components, prev, cx).await })
    }
}

impl Processing for Call {}

impl TryExecute for Call {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = cx.prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            self.func
                .execute(cx.clone().prev(&Some(prev_value.clone())))
                .await
        })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::{Call, Element, ElementRef};
    use proptest::prelude::*;

    impl Arbitrary for Call {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElementRef::Function], deep))
                .prop_map(|el| Call {
                    func: Box::new(el),
                    token: 0,
                })
                .boxed()
        }
    }
}
