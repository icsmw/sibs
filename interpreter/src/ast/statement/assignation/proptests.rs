use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Assignation {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
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
            },
        )
            .prop_map(move |(left, right)| Assignation {
                left: Box::new(left),
                token: Token::for_test(Kind::Equals),
                right: Box::new(right),
            })
            .boxed()
    }
}

test_node_reading!(assignation, Assignation, 10);
