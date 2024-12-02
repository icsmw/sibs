use crate::*;
use proptest::prelude::*;

impl Arbitrary for Closure {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                ArgumentDeclaration::arbitrary_with(deep + 1)
                    .prop_map(Declaration::ArgumentDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                1..5,
            ),
            Block::arbitrary_with(deep + 1)
                .prop_map(Statement::Block)
                .prop_map(Node::Statement)
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(|(args, block)| Closure {
                block: Box::new(block),
                args,
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
