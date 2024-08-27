use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, Scope, TokenGetter, TryExecute, Value, ValueRef,
        ValueTypeResult,
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

impl ExpectedValueType for Boolean {
    fn varification<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        Ok(())
    }

    fn linking<'a>(
        &'a self,
        _variables: &mut GlobalVariablesMap,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        Ok(())
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> ValueTypeResult {
        Ok(ValueRef::bool)
    }
}

impl TryExecute for Boolean {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [Value],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(Some(Value::bool(self.value))) })
    }
}

impl Execute for Boolean {}

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
