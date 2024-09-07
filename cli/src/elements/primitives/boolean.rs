use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, ExecutePinnedResult, ExpectedResult, TryExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, PrevValue, PrevValueExpectation, Scope,
        TokenGetter, TryExecute, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub token: usize,
}

impl TryDissect<Boolean> for Boolean {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Boolean>, LinkedErr<E>> {
        reader.move_to().any();
        if let Some(value) = reader.move_to().word(&[words::TRUE, words::FALSE]) {
            Ok(Some(Boolean {
                value: value == words::TRUE,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Boolean, Boolean> for Boolean {}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl Formation for Boolean {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Boolean {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Boolean {
    fn try_varification<'a>(
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
        _variables: &'a mut GlobalVariablesMap,
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
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl TryExecute for Boolean {
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
        Box::pin(async move { Ok(Value::bool(self.value)) })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::Boolean;
    use proptest::prelude::*;

    impl Arbitrary for Boolean {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(true), Just(false)]
                .prop_map(|value| Boolean { value, token: 0 })
                .boxed()
        }
    }
}
