use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Join {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            Command::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Expression(Expression::Command(v)))
                .boxed(),
            1..5,
        )
        .prop_map(move |commands| Join {
            commands,
            token: Token::for_test(Kind::Join),
        })
        .boxed()
    }
}

test_node_reading!(join, Join, 10);
