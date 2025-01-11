use crate::*;
use proptest::prelude::*;

impl Arbitrary for Task {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            proptest::option::of(Just(Token::for_test(Kind::Keyword(Keyword::Private)))),
            gens::rnd_kind_with(vec![KindId::Identifier]),
            prop::collection::vec(
                ArgumentDeclaration::arbitrary_with(deep + 1)
                    .prop_map(Declaration::ArgumentDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                1..5,
            ),
            Block::arbitrary_with(deep + 1)
                .prop_map(Statement::Block)
                .prop_map(Node::Statement)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(|(vis, name, args, block)| Task {
                vis,
                sig: Token::for_test(Kind::Keyword(Keyword::Task)),
                name: Token::for_test(name),
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
                block: Box::new(block),
                uuid: Uuid::new_v4(),
                args,
            })
            .boxed()
    }
}
