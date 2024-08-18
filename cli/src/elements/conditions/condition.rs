use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        AnyValue, Context, Execute, ExecutePinnedResult, Formation, FormationCursor, Scope,
        TokenGetter, TryExecute,
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

impl TryExecute for Condition {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [AnyValue],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            Ok(Some(AnyValue::bool(
                *self
                    .subsequence
                    .execute(owner, components, args, cx, sc, token)
                    .await?
                    .ok_or(E::NoValueFromSubsequence)?
                    .get::<bool>()
                    .ok_or(E::NoBoolValueFromSubsequence)?,
            )))
        })
    }
}

impl Execute for Condition {}

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
