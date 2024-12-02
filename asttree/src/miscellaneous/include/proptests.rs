use crate::*;
use proptest::prelude::*;

impl Arbitrary for Include {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        PrimitiveString::arbitrary()
            .prop_flat_map(|node| {
                LinkedNode::arbitrary_with(Node::Value(Value::PrimitiveString(node)))
            })
            .prop_map(|node| Include {
                token: Token::for_test(Kind::Keyword(Keyword::Mod)),
                node: Box::new(node),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
