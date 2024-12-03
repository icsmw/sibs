use crate::*;
use proptest::prelude::*;

impl Arbitrary for Module {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        PrimitiveString::arbitrary()
            .prop_map(|n| Node::Value(Value::PrimitiveString(n)))
            .prop_map(move |n| (n, 1))
            .prop_flat_map(LinkedNode::arbitrary_with)
            .prop_map(|node| Module {
                token: Token::for_test(Kind::Keyword(Keyword::Mod)),
                node: Box::new(node),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
