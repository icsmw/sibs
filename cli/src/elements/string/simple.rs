use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, Scope, TokenGetter, TryExecute, Value, ValueRef,
        ValueTypeResult,
    },
    reader::{Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct SimpleString {
    pub value: String,
    pub token: usize,
}

impl TryDissect<SimpleString> for SimpleString {
    fn try_dissect(reader: &mut Reader) -> Result<Option<SimpleString>, LinkedErr<E>> {
        reader.move_to().any();
        Ok(Some(SimpleString {
            value: reader.move_to().end(),
            token: reader.token()?.id,
        }))
    }
}

impl Dissect<SimpleString, SimpleString> for SimpleString {}

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl Formation for SimpleString {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for SimpleString {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for SimpleString {
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
        Ok(ValueRef::String)
    }
}

impl TryExecute for SimpleString {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [Value],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(Some(Value::String(self.value.to_string()))) })
    }
}

impl Execute for SimpleString {}

#[cfg(test)]
mod proptest {

    use crate::elements::SimpleString;
    use proptest::prelude::*;

    impl Arbitrary for SimpleString {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|value| SimpleString {
                    value: if value.is_empty() {
                        "min".to_owned()
                    } else {
                        value
                    },
                    token: 0,
                })
                .boxed()
        }
    }
}
