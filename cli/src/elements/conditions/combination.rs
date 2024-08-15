use tokio_util::sync::CancellationToken;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        AnyValue, Context, Execute, Formation, FormationCursor, ExecutePinnedResult, Scope,
        TokenGetter, TryExecute,
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

impl TryExecute for Combination {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        _args: &'a [String],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(Some(AnyValue::new(self.cmb.clone())?)) })
    }
}

impl Execute for Combination {}

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
