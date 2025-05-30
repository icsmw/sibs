use crate::*;

use proptest::prelude::*;

impl Arbitrary for BinaryExpGroup {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        BinaryExpSeq::arbitrary_with(deep + 1)
            .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
            .prop_map(move |n| (n, deep + 1))
            .prop_flat_map(LinkedNode::arbitrary_with)
            .boxed()
            .prop_map(|node| BinaryExpGroup {
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
                node: Box::new(node),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
