use crate::{
    entry::Component,
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: isize,
    pub token: usize,
}

impl Reading<Integer> for Integer {
    fn read(reader: &mut Reader) -> Result<Option<Integer>, LinkedErr<E>> {
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

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

impl term::Display for Integer {
    fn display(&self, term: &mut Term) {
        term.printnl(self.value);
    }
}

impl Operator for Integer {
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

    use crate::entry::Integer;
    use proptest::prelude::*;

    impl Arbitrary for Integer {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (std::isize::MIN..std::isize::MAX)
                .prop_map(|value| Integer { value, token: 0 })
                .boxed()
        }
    }
}
