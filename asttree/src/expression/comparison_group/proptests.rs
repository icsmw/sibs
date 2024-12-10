use crate::*;

use proptest::prelude::*;

impl Arbitrary for ComparisonGroup {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            ComparisonSeq::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with),
            prop::strategy::Union::new(vec![Just(Some(Token::for_test(Kind::Bang))), Just(None)]),
        )
            .boxed()
            .prop_map(move |(node, negation)| ComparisonGroup {
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
                node: Box::new(node),
                negation,
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
