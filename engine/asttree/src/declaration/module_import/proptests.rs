use crate::*;
use proptest::prelude::*;

impl Arbitrary for ModuleImport {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            PrimitiveString::arbitrary()
                .prop_map(|n| Node::Value(Value::PrimitiveString(n)))
                .prop_map(move |n| (n, 1))
                .prop_flat_map(LinkedNode::arbitrary_with),
            "[a-z][a-z0-9]*",
            if deep > PROPTEST_DEEP_FACTOR {
                prop::collection::vec(
                    prop_oneof![FunctionDeclaration::arbitrary_with(deep + 1)
                        .prop_map(Declaration::FunctionDeclaration)
                        .prop_map(Node::Declaration)
                        .prop_map(move |n| (n, deep + 1))
                        .prop_flat_map(LinkedNode::arbitrary_with)
                        .boxed(),],
                    1..2,
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
                        ModuleImport::arbitrary_with(PROPTEST_DEEP_FACTOR + 1)
                            .prop_map(Declaration::ModuleImport)
                            .prop_map(Node::Declaration)
                            .prop_map(move |n| (n, PROPTEST_DEEP_FACTOR + 1))
                            .prop_flat_map(LinkedNode::arbitrary_with)
                            .boxed(),
                    ],
                    1..2,
                )
                .boxed()
            },
        )
            .prop_map(|(node, name, nodes)| ModuleImport {
                sig: Token::for_test(Kind::Keyword(Keyword::Mod)),
                from: Token::for_test(Kind::Identifier(String::from("from"))),
                node: Box::new(node),
                name,
                body: std::sync::Arc::new(ModuleBody {
                    nodes,
                    source: Uuid::new_v4(),
                }),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
