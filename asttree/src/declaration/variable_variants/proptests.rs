use crate::*;
use proptest::prelude::*;

impl Arbitrary for VariableVariants {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            prop::strategy::Union::new(vec![
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
                PrimitiveString::arbitrary()
                    .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                    .boxed(),
            ])
            .prop_map(LinkedNode::from_node),
            1..5,
        )
        .prop_map(|variants| {
            let token = Token::for_test(Kind::Colon);
            VariableVariants {
                variants,
                token,
                uuid: Uuid::new_v4(),
            }
        })
        .boxed()
    }
}
