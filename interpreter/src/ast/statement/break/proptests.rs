use crate::*;

use lexer::{gens, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Break {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::Break)
            .boxed()
            .prop_map(move |knd| Break {
                token: Token::for_test(knd),
            })
            .boxed()
    }
}

test_node_reading!(r#break, Break, 10);
