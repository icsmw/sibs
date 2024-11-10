use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Each {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            if deep > 5 {
                prop::strategy::Union::new(vec![Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed()])
            } else {
                prop::strategy::Union::new(vec![
                    Array::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::Array(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                ])
            },
            Block::arbitrary()
                .prop_map(|v| Node::Statement(Statement::Block(v)))
                .boxed(),
        )
            .prop_map(move |(element, index, elements, block)| Each {
                element: Box::new(element),
                index: Box::new(index),
                elements: Box::new(elements),
                block: Box::new(block),
                token: Token::for_test(Kind::Each),
            })
            .boxed()
    }
}

test_node_reading!(each, Each, 10);
