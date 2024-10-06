use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Accessor {
    pub index: Box<Element>,
    pub token: usize,
}

impl TryDissect<Accessor> for Accessor {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Accessor>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Accessor);
        Ok(
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let Some(el) = Element::include(
                    &mut inner,
                    &[
                        ElementRef::Integer,
                        ElementRef::Function,
                        ElementRef::VariableName,
                    ],
                )?
                else {
                    return Err(E::NoElementAccessor.linked(&close(reader)));
                };
                Some(Accessor {
                    index: Box::new(el),
                    token: close(reader),
                })
            } else {
                None
            },
        )
    }
}

impl Dissect<Accessor, Accessor> for Accessor {}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.index)
    }
}

impl Formation for Accessor {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!("[{}]", self.index)
    }
}

impl TokenGetter for Accessor {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Accessor {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { self.index.verification(owner, components, prev, cx).await })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { self.index.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            Ok(match &prev_value.value {
                ValueRef::String => ValueRef::String,
                ValueRef::Vec(ty) => *ty.clone(),
                el => {
                    return Err(
                        operator::E::NotSupportedTypeByAccessor(el.to_owned()).linked(&self.token)
                    )?;
                }
            })
        })
    }
}

impl Processing for Accessor {}

impl TryExecute for Accessor {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Some(prev_value) = cx.prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            let n = self
                .index
                .execute(cx)
                .await?
                .as_num()
                .ok_or(operator::E::FailToExtractAccessorIndex.by(&*self.index))?;
            if n < 0 {
                return Err(operator::E::NegativeAccessorIndex(n).by(&*self.index));
            }
            let n = n as usize;
            Ok(match &prev_value.value {
                Value::String(v) => Value::String(
                    v.chars()
                        .nth(n)
                        .ok_or(operator::E::OutOfBounds(v.chars().count(), n).linked(&self.token))?
                        .to_string(),
                ),
                Value::Vec(v) => v
                    .get(n)
                    .map(|v| v.duplicate())
                    .ok_or(operator::E::OutOfBounds(v.len(), n).linked(&self.token))?,
                _ => Err(
                    operator::E::AccessByIndexNotSupported(prev_value.value.to_string())
                        .linked(&self.token),
                )?,
            })
        })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::{Accessor, Element, ElementRef};
    use proptest::prelude::*;

    impl Arbitrary for Accessor {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((
                vec![
                    ElementRef::Function,
                    ElementRef::VariableName,
                    ElementRef::Integer,
                ],
                deep,
            ))
            .prop_map(|el| Accessor {
                index: Box::new(el),
                token: 0,
            })
            .boxed()
        }
    }
}
