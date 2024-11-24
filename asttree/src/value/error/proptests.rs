use crate::*;
use proptest::prelude::*;

impl Arbitrary for Error {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::strategy::Union::new(vec![
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            Number::arbitrary()
                .prop_map(|v| Node::Value(Value::Number(v)))
                .boxed(),
            PrimitiveString::arbitrary()
                .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                .boxed(),
            InterpolatedString::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Value(Value::InterpolatedString(v)))
                .boxed(),
        ])
        .prop_map(move |node| Error {
            node: Box::new(node),
            token: Token::for_test(Kind::Identifier(String::from("Error"))),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
