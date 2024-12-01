use crate::*;
use proptest::prelude::*;

impl Arbitrary for FunctionDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            gens::rnd_kind_with(vec![KindId::Identifier]),
            prop::collection::vec(
                ArgumentDeclaration::arbitrary_with(deep + 1)
                    .prop_map(Declaration::ArgumentDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_map(LinkedNode::from_node)
                    .boxed(),
                1..5,
            ),
            Block::arbitrary_with(deep + 1)
                .prop_map(Statement::Block)
                .prop_map(Node::Statement)
                .prop_map(LinkedNode::from_node)
                .boxed(),
        )
            .prop_map(|(name, args, block)| FunctionDeclaration {
                sig: Token::for_test(Kind::Keyword(Keyword::Fn)),
                name: Token::for_test(name),
                block: Box::new(block),
                args,
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
