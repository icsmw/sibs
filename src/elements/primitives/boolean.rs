use crate::{
    elements::{Component, ElTarget},
    error::LinkedErr,
    inf::{
        term, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Term,
    },
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
    pub token: usize,
}

impl Reading<Boolean> for Boolean {
    fn read(reader: &mut Reader) -> Result<Option<Boolean>, LinkedErr<E>> {
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

impl term::Display for Boolean {
    fn display(&self, term: &mut Term) {
        term.printnl(self.value);
    }
}

impl Operator for Boolean {
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
        Box::pin(async move { Ok(Some(AnyValue::new(self.value))) })
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
