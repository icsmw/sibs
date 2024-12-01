use crate::*;
use proptest::prelude::*;

impl Arbitrary for OneOf {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            Command::arbitrary_with(deep + 1)
                .prop_map(|v| LinkedNode::from_node(Node::Expression(Expression::Command(v))))
                .boxed(),
            1..5,
        )
        .prop_map(|commands| OneOf {
            commands,
            open: Token::for_test(Kind::LeftParen),
            close: Token::for_test(Kind::RightParen),
            token: Token::for_test(Kind::Keyword(Keyword::OneOf)),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
