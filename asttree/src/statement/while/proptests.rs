use crate::*;
use proptest::prelude::*;

impl Arbitrary for While {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            ComparisonSeq::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            Block::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Statement(Statement::Block(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(move |(comparison, block)| While {
                comparison: Box::new(comparison),
                block: Box::new(block),
                token: Token::for_test(Kind::Keyword(Keyword::While)),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
