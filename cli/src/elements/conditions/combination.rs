use crate::{
    elements::{Element, TokenGetter},
    error::LinkedErr,
    inf::{
        Context, ExecuteContext, ExecutePinnedResult, ExpectedResult, Formation, FormationCursor,
        LinkingResult, PrevValueExpectation, Processing, TryExecute, TryExpectedValueType, Value,
        ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cmb {
    And,
    Or,
}

impl fmt::Display for Cmb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => words::AND,
                Self::Or => words::OR,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Combination {
    pub cmb: Cmb,
    pub token: usize,
}

impl TryDissect<Combination> for Combination {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Combination>, LinkedErr<E>> {
        if reader.move_to().expression(&[words::AND]).is_some() {
            Ok(Some(Combination {
                cmb: Cmb::And,
                token: reader.token()?.id,
            }))
        } else if reader.move_to().expression(&[words::OR]).is_some() {
            Ok(Some(Combination {
                cmb: Cmb::Or,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Combination, Combination> for Combination {}

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} ", self.cmb)
    }
}

impl Formation for Combination {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}

impl TokenGetter for Combination {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Combination {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}

impl Processing for Combination {}

impl TryExecute for Combination {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::Cmb(self.cmb.clone())) })
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::{Cmb, Combination};
    use proptest::prelude::*;

    impl Arbitrary for Combination {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Combination {
                    cmb: Cmb::And,
                    token: 0
                }),
                Just(Combination {
                    cmb: Cmb::Or,
                    token: 0
                }),
            ]
            .boxed()
        }
    }
}
