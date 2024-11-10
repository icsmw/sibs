use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Optional {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            ComparisonSeq::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Expression(Expression::ComparisonSeq(v)))
                .boxed(),
            if deep > 5 {
                prop::strategy::Union::new(vec![
                    Break::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Break(v)))
                        .boxed(),
                    Return::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Return(v)))
                        .boxed(),
                ])
            } else {
                prop::strategy::Union::new(vec![
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    Command::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::Command(v)))
                        .boxed(),
                    Block::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Block(v)))
                        .boxed(),
                    Break::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Break(v)))
                        .boxed(),
                    Return::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Return(v)))
                        .boxed(),
                    Loop::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Loop(v)))
                        .boxed(),
                    For::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::For(v)))
                        .boxed(),
                    While::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::While(v)))
                        .boxed(),
                    Assignation::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Assignation(v)))
                        .boxed(),
                    Each::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Each(v)))
                        .boxed(),
                    // Join::arbitrary_with(deep + 1)
                    //     .prop_map(|v| Node::Statement(Statement::Join(v)))
                    //     .boxed(),
                    // OneOf::arbitrary_with(deep + 1)
                    //     .prop_map(|v| Node::Statement(Statement::OneOf(v)))
                    //     .boxed(),
                ])
            },
        )
            .prop_map(move |(comparison, action)| Optional {
                comparison: Box::new(comparison),
                action: Box::new(action),
                token: Token::for_test(Kind::DoubleArrow),
            })
            .boxed()
    }
}

test_node_reading!(optional, Optional, 10);
