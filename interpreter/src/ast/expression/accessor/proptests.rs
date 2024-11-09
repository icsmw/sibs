use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Accessor {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::strategy::Union::new(vec![
            Number::arbitrary()
                .prop_map(|v| Node::Value(Value::Number(v)))
                .boxed(),
            FunctionCall::arbitrary_with(0)
                .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                .boxed(),
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            BinaryExpSeq::arbitrary()
                .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                .boxed(),
            If::arbitrary()
                .prop_map(|v| Node::Statement(Statement::If(v)))
                .boxed(),
        ])
        .prop_map(move |node| Accessor {
            node: Box::new(node),
        })
        .boxed()
    }
}

test_node_reading!(accessor, Accessor, 10);
