use crate::*;
use lexer::{gens, Keyword, Kind, KindId, Token};
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
                    .boxed(),
                1..5,
            ),
            Block::arbitrary_with(deep + 1)
                .prop_map(Statement::Block)
                .prop_map(Node::Statement)
                .boxed(),
        )
            .prop_map(|(name, args, block)| FunctionDeclaration {
                sig: Token::for_test(Kind::Keyword(Keyword::Fn)),
                name: Token::for_test(name),
                block: Box::new(block),
                args,
            })
            .boxed()
    }
}

test_node_reading!(function_declaration, FunctionDeclaration, 10);
