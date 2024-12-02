use crate::*;
use proptest::prelude::*;

impl Arbitrary for Return {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        proptest::option::of(
            if deep > PROPTEST_DEEP_FACTOR {
                prop::strategy::Union::new(vec![
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                ])
            } else {
                prop::strategy::Union::new(vec![
                    ComparisonSeq::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                        .boxed(),
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                    InterpolatedString::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::InterpolatedString(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    Error::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::Error(v)))
                        .boxed(),
                ])
            }
            .prop_flat_map(LinkedNode::arbitrary_with),
        )
        .prop_map(|node| Return {
            token: Token::for_test(Kind::Keyword(Keyword::Return)),
            node: node.map(Box::new),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
