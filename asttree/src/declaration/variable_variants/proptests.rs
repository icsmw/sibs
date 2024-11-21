use crate::*;
use lexer::{Kind, Token};
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
            ]),
            1..5,
        )
        .prop_map(|types| {
            let token = Token::for_test(Kind::Colon);
            VariableVariants { types, token }
        })
        .boxed()
    }
}
