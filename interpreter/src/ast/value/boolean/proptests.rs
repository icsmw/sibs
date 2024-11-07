use crate::*;

use lexer::{gens, Kind, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Boolean {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::rnd_kind_with(vec![KindId::True, KindId::False])
            .prop_filter_map("Expected: Kind::True | Kind::False", |knds| {
                knds.first()
                    .filter(|knd| matches!(knd, Kind::True | Kind::False))
                    .map(|knd| match knd {
                        Kind::True => Boolean {
                            inner: true,
                            token: Token::for_test(Kind::True),
                        },
                        Kind::False => Boolean {
                            inner: false,
                            token: Token::for_test(Kind::False),
                        },
                        _ => panic!("Expected: Kind::True | Kind::False"),
                    })
            })
            .boxed()
    }
}
