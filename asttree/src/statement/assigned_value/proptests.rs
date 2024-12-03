use crate::*;
use proptest::prelude::*;

impl Arbitrary for AssignedValue {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
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
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
            ])
        } else {
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
                InterpolatedString::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Value(Value::InterpolatedString(v)))
                    .boxed(),
                Array::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Value(Value::Array(v)))
                    .boxed(),
                FunctionCall::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                    .boxed(),
                ComparisonSeq::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                    .boxed(),
                Command::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::Command(v)))
                    .boxed(),
                TaskCall::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::TaskCall(v)))
                    .boxed(),
                BinaryExpSeq::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                If::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Statement(Statement::If(v)))
                    .boxed(),
            ])
        }
        .prop_map(move |n| (n, deep + 1))
        .prop_flat_map(LinkedNode::arbitrary_with)
        .prop_map(move |node| AssignedValue {
            token: Token::for_test(Kind::Equals),
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
