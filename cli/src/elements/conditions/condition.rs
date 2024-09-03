use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, PrevValue, Scope, TokenGetter,
        TryExecute, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Condition {
    pub subsequence: Box<Element>,
    pub token: usize,
}

impl TryDissect<Condition> for Condition {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Condition>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Condition);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            if let Some(el) = Element::include(&mut inner, &[ElTarget::Subsequence])? {
                Ok(Some(Condition {
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

impl Dissect<Condition, Condition> for Condition {}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.subsequence)
    }
}

impl Formation for Condition {
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
                    .format(&mut cursor.reown(Some(ElTarget::Condition)))
            )
        } else {
            format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
        }
    }
}

impl TokenGetter for Condition {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Condition {
    fn varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            self.subsequence
                .linking(variables, owner, components, cx)
                .await
        })
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl TryExecute for Condition {
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
    use crate::elements::{Condition, ElTarget, Element};
    use proptest::prelude::*;

    impl Arbitrary for Condition {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElTarget::Subsequence], deep))
                .prop_map(|subsequence| Condition {
                    subsequence: Box::new(subsequence),
                    token: 0,
                })
                .boxed()
        }
    }
}
