use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope, TokenGetter,
        TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct IfCondition {
    pub subsequence: Box<Element>,
    pub token: usize,
}

impl TryDissect<IfCondition> for IfCondition {
    fn try_dissect(reader: &mut Reader) -> Result<Option<IfCondition>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::IfCondition);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            if let Some(el) = Element::include(&mut inner, &[ElTarget::IfSubsequence])? {
                Ok(Some(IfCondition {
                    subsequence: Box::new(el),
                    token: close(reader),
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Dissect<IfCondition, IfCondition> for IfCondition {}

impl fmt::Display for IfCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.subsequence)
    }
}

impl Formation for IfCondition {
    fn elements_count(&self) -> usize {
        self.subsequence.elements_count()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.elements_count() > cursor.max_elements()
            || self.to_string().len() > cursor.max_len()
        {
            format!(
                "{}({})",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                self.subsequence
                    .format(&mut cursor.reown(Some(ElTarget::IfCondition)))
            )
        } else {
            format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
        }
    }
}

impl TokenGetter for IfCondition {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for IfCondition {
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
        Box::pin(async move { self.subsequence.linking(owner, components, prev, cx).await })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl TryExecute for IfCondition {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            Ok(Value::bool(
                *self
                    .subsequence
                    .execute(owner, components, args, prev, cx, sc, token)
                    .await?
                    .get::<bool>()
                    .ok_or(E::NoBoolValueFromSubsequence)?,
            ))
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::elements::{ElTarget, Element, IfCondition};
    use proptest::prelude::*;

    impl Arbitrary for IfCondition {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElTarget::IfSubsequence], deep))
                .prop_map(|subsequence| IfCondition {
                    subsequence: Box::new(subsequence),
                    token: 0,
                })
                .boxed()
        }
    }
}
