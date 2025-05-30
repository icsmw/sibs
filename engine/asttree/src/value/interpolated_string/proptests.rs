use std::{ops::RangeInclusive, vec};

use crate::*;
use gens::gen_string;
use proptest::prelude::*;

impl Arbitrary for InterpolatedStringPart {
    type Parameters = (u8, bool);

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((deep, lit): Self::Parameters) -> Self::Strategy {
        if lit {
            return gen_string(RangeInclusive::new(1, 100))
                .prop_map(|str| {
                    InterpolatedStringPart::Literal(Token::for_test(Kind::Literal(
                        str.chars()
                            .map(|ch| {
                                if ch == '{' || ch == '}' || ch == '\\' || ch == '\'' {
                                    "_".to_string()
                                } else {
                                    ch.to_string()
                                }
                            })
                            .collect::<String>(),
                    )))
                })
                .boxed();
        }
        if deep > PROPTEST_DEEP_FACTOR {
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
            ])
        } else {
            prop::strategy::Union::new(vec![
                If::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Statement(Statement::If(v)))
                    .boxed(),
                ComparisonSeq::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                BinaryExpSeq::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
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
            ])
        }
        .prop_map(move |n| (n, deep + 1))
        .prop_flat_map(LinkedNode::arbitrary_with)
        .prop_map(|n| {
            InterpolatedStringPart::Expression(
                Token::for_test(Kind::LeftBrace),
                n,
                Token::for_test(Kind::RightBrace),
            )
        })
        .boxed()
    }
}

impl Arbitrary for InterpolatedString {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            proptest::collection::vec(
                InterpolatedStringPart::arbitrary_with((deep + 1, true)),
                0..10,
            ),
            proptest::collection::vec(
                InterpolatedStringPart::arbitrary_with((deep + 1, false)),
                0..10,
            ),
        )
            .prop_map(|(mut lits, mut exps)| {
                let mut nodes = vec![InterpolatedStringPart::Open(Token::for_test(
                    Kind::SingleQuote,
                ))];
                if lits.len() > exps.len() {
                    for exp in exps.into_iter() {
                        nodes.push(exp);
                        nodes.push(lits.remove(0));
                    }
                } else {
                    for lit in lits.into_iter() {
                        nodes.push(lit);
                        nodes.push(exps.remove(0));
                    }
                }
                nodes.push(InterpolatedStringPart::Close(Token::for_test(
                    Kind::SingleQuote,
                )));
                InterpolatedString {
                    nodes,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
