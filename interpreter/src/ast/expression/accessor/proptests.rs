use crate::*;

use proptest::prelude::*;

impl Arbitrary for Accessor {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
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
            If::arbitrary_with(deep + 1)
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
