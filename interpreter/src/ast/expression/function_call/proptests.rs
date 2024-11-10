use crate::*;

use lexer::{gens, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for FunctionCall {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(gens::kind(KindId::Identifier).boxed(), 1..5),
            if deep > 5 {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        ComparisonSeq::arbitrary()
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
                    ]),
                    1..5,
                )
            } else {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        ComparisonSeq::arbitrary()
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
                    ]),
                    1..5,
                )
            },
        )
            .prop_map(move |(idents, args)| FunctionCall {
                reference: idents
                    .into_iter()
                    .map(|knd| (knd.to_string(), Token::for_test(knd)))
                    .collect::<Vec<(String, Token)>>(),
                args,
            })
            .boxed()
    }
}

test_node_reading!(function_call, FunctionCall, 10);
