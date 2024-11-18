use crate::*;
use lexer::{gens, Keyword, Kind, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Component {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            gens::rnd_kind_with(vec![KindId::Identifier]),
            prop::collection::vec(gens::kind(KindId::Identifier), 1..5),
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Task::arbitrary_with(deep + 1)
                        .prop_map(Root::Task)
                        .prop_map(Node::Root)
                        .boxed(),
                    Gatekeeper::arbitrary_with(deep + 1)
                        .prop_map(ControlFlowModifier::Gatekeeper)
                        .prop_map(Node::ControlFlowModifier)
                        .boxed(),
                    Comment::arbitrary()
                        .prop_map(|v| Node::Miscellaneous(Miscellaneous::Comment(v)))
                        .boxed(),
                    Meta::arbitrary()
                        .prop_map(|v| Node::Miscellaneous(Miscellaneous::Meta(v)))
                        .boxed(),
                ]),
                1..5,
            ),
        )
            .prop_map(|(name, path, nodes)| Component {
                sig: Token::for_test(Kind::Keyword(Keyword::Component)),
                name: Token::for_test(name),
                path: path
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join("/"),
                nodes,
                open_bl: Token::for_test(Kind::LeftBrace),
                close_bl: Token::for_test(Kind::RightBrace),
            })
            .boxed()
    }
}

test_node_reading!(component, Component, 10);
