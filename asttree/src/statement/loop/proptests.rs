use crate::*;
use proptest::prelude::*;

impl Arbitrary for Loop {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Block::arbitrary_with(deep + 1)
            .prop_map(|n| Node::Statement(Statement::Block(n)))
            .prop_map(move |n| (n, deep + 1))
            .prop_flat_map(LinkedNode::arbitrary_with)
            .boxed()
            .prop_map(move |block| Loop {
                block: Box::new(block),
                token: Token::for_test(Kind::Keyword(Keyword::Loop)),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
