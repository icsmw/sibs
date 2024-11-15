use crate::*;
use lexer::{gens, Keyword, Kind, KindId, Token};
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
                    .boxed(),
                1..5,
            ),
            Block::arbitrary_with(deep + 1)
                .prop_map(Statement::Block)
                .prop_map(Node::Statement)
                .boxed(),
        )
            .prop_map(|(vis, name, args, block)| Task {
                vis,
                sig: Token::for_test(Kind::Keyword(Keyword::Task)),
                name: Token::for_test(name),
                block: Box::new(block),
                args,
            })
            .boxed()
    }
}

test_node_reading!(task, Task, 10);

// test_node_reading_case!(task_case, Task, "task name() { f = 4; }");
