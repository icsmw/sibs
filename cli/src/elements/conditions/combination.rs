use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
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

impl Reading<Combination> for Combination {
    fn read(reader: &mut Reader) -> Result<Option<Combination>, LinkedErr<E>> {
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

impl Operator for Combination {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [String],
        _cx: Context,
        _sc: Scope,
    ) -> OperatorPinnedResult {
        Box::pin(async move { Ok(Some(AnyValue::new(self.cmb.clone()))) })
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
