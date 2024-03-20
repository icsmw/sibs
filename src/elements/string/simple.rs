use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
    },
    reader::{Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct SimpleString {
    pub value: String,
    pub token: usize,
}

impl Reading<SimpleString> for SimpleString {
    fn read(reader: &mut Reader) -> Result<Option<SimpleString>, LinkedErr<E>> {
        Ok(Some(SimpleString {
            value: reader.move_to().end(),
            token: reader.token()?.id,
        }))
    }
}

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl Operator for SimpleString {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _inputs: &'a [String],
        _cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move { Ok(Some(AnyValue::new(self.value.to_string()))) })
    }
}

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
