use crate::*;

use lexer::{gens, Kind, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Boolean {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::rnd_kind_with(vec![KindId::True, KindId::False])
            .prop_filter_map("Expected: Kind::True | Kind::False", |knd| match knd {
                Kind::True => Some(Boolean {
                    inner: true,
                    token: Token::for_test(Kind::True),
                }),
                Kind::False => Some(Boolean {
                    inner: false,
                    token: Token::for_test(Kind::False),
                }),
                _ => None,
            })
            .boxed()
    }
}

test_node_reading!(boolean, Boolean, 10);
