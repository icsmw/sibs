use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for For {
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
            if deep > PROPTEST_DEEP_FACTOR {
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Range::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Range(v)))
                        .boxed(),
                ])
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
                    Range::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Range(v)))
                        .boxed(),
                ])
            },
            Block::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Statement(Statement::Block(v)))
                .boxed(),
        )
            .prop_map(move |(element, index, elements, block)| For {
                element: Box::new(element),
                index: Box::new(index),
                elements: Box::new(elements),
                block: Box::new(block),
                token_for: Token::for_test(Kind::For),
                token_in: Token::for_test(Kind::In),
            })
            .boxed()
    }
}

test_node_reading!(r#for, For, 10);
