use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Condition {
    pub subsequence: Box<Element>,
    pub token: usize,
}

impl Reading<Condition> for Condition {
    fn read(reader: &mut Reader) -> Result<Option<Condition>, LinkedErr<E>> {
        let close = reader.open_token();
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

impl Operator for Condition {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            Ok(Some(AnyValue::new(
                *self
                    .subsequence
                    .execute(owner, components, args, cx)
                    .await?
                    .ok_or(E::NoValueFromSubsequence)?
                    .get_as::<bool>()
                    .ok_or(E::NoBoolValueFromSubsequence)?,
            )))
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
