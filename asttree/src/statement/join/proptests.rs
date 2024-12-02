use crate::*;
use proptest::prelude::*;

impl Arbitrary for Join {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            Command::arbitrary_with(deep + 1)
                .prop_flat_map(|v| {
                    LinkedNode::arbitrary_with(Node::Expression(Expression::Command(v)))
                })
                .boxed(),
            1..5,
        )
        .prop_map(move |commands| Join {
            open: Token::for_test(Kind::LeftParen),
            close: Token::for_test(Kind::RightParen),
            commands,
            token: Token::for_test(Kind::Keyword(Keyword::Join)),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
