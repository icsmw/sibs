use crate::*;
use proptest::prelude::*;

impl Arbitrary for Component {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            gens::rnd_kind_with(vec![KindId::Identifier]),
            prop::collection::vec(gens::kind(KindId::Identifier), 1..5),
            prop::collection::vec(
                Task::arbitrary_with(deep + 1)
                    .prop_map(Root::Task)
                    .prop_map(Node::Root)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
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
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
