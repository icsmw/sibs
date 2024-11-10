use crate::*;

use proptest::prelude::*;

impl Arbitrary for Comparison {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > 5 {
            (
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                ]),
                ComparisonOp::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::ComparisonOp(v)))
                    .boxed(),
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                ]),
            )
        } else {
            (
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
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
                ]),
                ComparisonOp::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::ComparisonOp(v)))
                    .boxed(),
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
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
                ]),
            )
        }
        .prop_map(move |(left, operator, right)| Comparison {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        })
        .boxed()
    }
}

test_node_reading!(comparison, Comparison, 10);
