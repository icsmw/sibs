use crate::*;

use proptest::prelude::*;

impl Arbitrary for Array {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > 5 {
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    BinaryExpSeq::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                        .boxed(),
                    ComparisonSeq::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                        .boxed(),
                    FunctionCall::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
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
                ]),
                1..5,
            )
        } else {
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    BinaryExpSeq::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                        .boxed(),
                    ComparisonSeq::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                        .boxed(),
                    FunctionCall::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
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
                ]),
                1..5,
            )
        }
        .prop_map(move |els| Array { els })
        .boxed()
    }
}

test_node_reading!(array, Array, 10);
