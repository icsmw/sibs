use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope,
        TokenGetter, TryExecute, TryExpectedValueType, Value, VerificationResult,
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
        let close = reader.open_token(ElTarget::Accessor);
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
                        ElTarget::Integer,
                        ElTarget::Function,
                        ElTarget::VariableName,
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
    fn try_varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { self.index.varification(owner, components, prev, cx).await })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { self.index.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { self.index.expected(owner, components, prev, cx).await })
    }
}

impl TryExecute for Accessor {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let Some(prev_value) = prev else {
                return Err(operator::E::CallPPMWithoutPrevValue.linked(&self.token));
            };
            let n = self
                .index
                .execute(owner, components, args, prev, cx, sc, token)
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

    use crate::elements::{Accessor, ElTarget, Element};
    use proptest::prelude::*;

    impl Arbitrary for Accessor {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((
                vec![
                    ElTarget::Function,
                    ElTarget::VariableName,
                    ElTarget::Integer,
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
