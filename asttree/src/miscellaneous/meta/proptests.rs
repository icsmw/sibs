use crate::*;
use lexer::{gens, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Meta {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::Meta)
            .prop_map(|kind| Meta {
                token: Token::for_test(kind),
            })
            .boxed()
    }
}
