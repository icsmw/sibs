use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, ExecutePinnedResult, ExpectedResult, Formation, FormationCursor, LinkingResult,
        PrevValue, PrevValueExpectation, Scope, TokenGetter, TryExecute, TryExpectedValueType,
        Value, ValueRef, VerificationResult,
    },
    reader::{Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: isize,
    pub token: usize,
}

impl TryDissect<Integer> for Integer {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Integer>, LinkedErr<E>> {
        reader.move_to().any();
        if let Some(value) = reader.move_to().none_numeric() {
            Ok(Some(Integer {
                value: value
                    .parse::<isize>()
                    .map_err(|e| E::IntegerParsingError(e.to_string()).by_reader(reader))?,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Integer, Integer> for Integer {}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl Formation for Integer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Integer {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Integer {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            Ok(ValueRef::OneOf(vec![
                ValueRef::u8,
                ValueRef::u16,
                ValueRef::u32,
                ValueRef::u64,
                ValueRef::u128,
                ValueRef::usize,
                ValueRef::i8,
                ValueRef::i16,
                ValueRef::i32,
                ValueRef::i64,
                ValueRef::i128,
                ValueRef::isize,
            ]))
        })
    }
}

impl TryExecute for Integer {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Element>,
        _components: &'a [Element],
        _args: &'a [Value],
        _prev: &'a Option<PrevValue>,
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(Value::isize(self.value)) })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::Integer;
    use proptest::prelude::*;

    impl Arbitrary for Integer {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (isize::MIN..isize::MAX)
                .prop_map(|value| Integer { value, token: 0 })
                .boxed()
        }
    }
}
