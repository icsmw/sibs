use crate::*;
use proptest::prelude::*;

impl Arbitrary for Module {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            gens::kind(KindId::Identifier).prop_filter("avoid from", |knd| {
                if let Kind::Identifier(vl) = knd {
                    vl != "from"
                } else {
                    false
                }
            }),
            if deep > PROPTEST_DEEP_FACTOR {
                prop::collection::vec(
                    prop_oneof![
                        FunctionDeclaration::arbitrary_with(deep + 1)
                            .prop_map(Declaration::FunctionDeclaration)
                            .prop_map(Node::Declaration)
                            .prop_map(move |n| (n, deep + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed(),
                        ModuleDeclaration::arbitrary()
                            .prop_map(Declaration::ModuleDeclaration)
                            .prop_map(Node::Declaration)
                            .prop_map(move |n| (n, deep + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed(),
                    ],
                    1..10,
                )
                .boxed()
            } else {
                prop::collection::vec(
                    prop_oneof![
                        FunctionDeclaration::arbitrary_with(deep + 1)
                            .prop_map(Declaration::FunctionDeclaration)
                            .prop_map(Node::Declaration)
                            .prop_map(move |n| (n, deep + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed(),
                        ModuleDeclaration::arbitrary()
                            .prop_map(Declaration::ModuleDeclaration)
                            .prop_map(Node::Declaration)
                            .prop_map(move |n| (n, deep + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed(),
                        Module::arbitrary_with(PROPTEST_DEEP_FACTOR + 1)
                            .prop_map(Root::Module)
                            .prop_map(Node::Root)
                            .prop_map(move |n| (n, PROPTEST_DEEP_FACTOR + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed()
                    ],
                    1..10,
                )
                .boxed()
            },
        )
            .prop_map(|(name, nodes)| Module {
                sig: Token::for_test(Kind::Keyword(Keyword::Mod)),
                name: Token::for_test(name),
                open: Token::for_test(Kind::LeftBrace),
                close: Token::for_test(Kind::RightBrace),
                nodes,
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
