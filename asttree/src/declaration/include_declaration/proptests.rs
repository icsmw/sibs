use crate::*;
use proptest::prelude::*;

impl Arbitrary for IncludeDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        PrimitiveString::arbitrary()
            .prop_map(|n| Node::Value(Value::PrimitiveString(n)))
            .prop_map(move |n| (n, 1))
            .prop_flat_map(LinkedNode::arbitrary_with)
            .prop_map(|node| IncludeDeclaration {
                sig: Token::for_test(Kind::Keyword(Keyword::Include)),
                from: Token::for_test(Kind::Identifier(String::from("from"))),
                node: Box::new(node),
                root: Box::new(LinkedNode::from_node(Node::Root(Root::Anchor(Anchor {
                    nodes: Vec::new(),
                    uuid: Uuid::new_v4(),
                })))),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
