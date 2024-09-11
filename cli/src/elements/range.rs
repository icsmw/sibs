use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope,
        TokenGetter, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Range {
    pub from: Box<Element>,
    pub to: Box<Element>,
    pub token: usize,
}

impl TryDissect<Range> for Range {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Range>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Range);
        let Some(from) = Element::include(reader, &[ElTarget::VariableName, ElTarget::Integer])?
        else {
            return Ok(None);
        };
        if reader.move_to().word_any(&[words::RANGE]).is_none() {
            return Ok(None);
        }
        let Some(to) = Element::include(reader, &[ElTarget::VariableName, ElTarget::Integer])?
        else {
            return Err(E::NoEndRangeBorder.by_reader(reader));
        };
        Ok(Some(Self {
            from: Box::new(from),
            to: Box::new(to),
            token: close(reader),
        }))
    }
}

impl Dissect<Range, Range> for Range {}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.from, self.to)
    }
}

impl Formation for Range {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!("{}..{}", self.from, self.to)
    }
}

impl TokenGetter for Range {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Range {
    fn try_varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            if self
                .from
                .expected(owner, components, prev, cx)
                .await?
                .is_compatible(&ValueRef::Numeric)
            {
                return Err(operator::E::ExpectedNumericValue.linked(&self.from.token()));
            }
            if self
                .to
                .expected(owner, components, prev, cx)
                .await?
                .is_compatible(&ValueRef::Numeric)
            {
                return Err(operator::E::ExpectedNumericValue.linked(&self.to.token()));
            }
            self.from.varification(owner, components, prev, cx).await?;
            self.to.varification(owner, components, prev, cx).await
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            self.from.linking(owner, components, prev, cx).await?;
            self.to.linking(owner, components, prev, cx).await
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::Vec(Box::new(ValueRef::usize))) })
    }
}

impl TryExecute for Range {
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
            let from = self
                .from
                .execute(
                    owner,
                    components,
                    args,
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .as_num()
                .ok_or(operator::E::ExpectedNumericValue.linked(&self.from.token()))?;
            let to = self
                .to
                .execute(owner, components, args, prev, cx, sc, token)
                .await?
                .as_num()
                .ok_or(operator::E::ExpectedNumericValue.linked(&self.to.token()))?;
            Ok(Value::Vec(vec![Value::isize(from), Value::isize(to)]))
        })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::{ElTarget, Element, Range};
    use proptest::prelude::*;

    impl Arbitrary for Range {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElTarget::VariableName, ElTarget::Integer], deep)),
                Element::arbitrary_with((vec![ElTarget::VariableName, ElTarget::Integer], deep)),
            )
                .prop_map(|(from, to)| Range {
                    from: Box::new(from),
                    to: Box::new(to),
                    token: 0,
                })
                .boxed()
        }
    }
}
