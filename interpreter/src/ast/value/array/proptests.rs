use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Array {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > PROPTEST_DEEP_FACTOR {
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
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
                    BinaryExpSeq::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                        .boxed(),
                    ComparisonSeq::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    Command::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::Command(v)))
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
                ]),
                1..5,
            )
        }
        .prop_map(move |els| Array {
            els,
            open: Token::for_test(Kind::LeftBracket),
            close: Token::for_test(Kind::RightBracket),
        })
        .boxed()
    }
}

test_node_reading!(array, Array, 10);
