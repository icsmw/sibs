use crate::*;

use proptest::prelude::*;

impl Arbitrary for BinaryExp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            prop::strategy::Union::new(vec![
                FunctionCall::arbitrary_with(PROPTEST_DEEP_FACTOR + 1)
                    .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
            ])
            .prop_map(move |n| (n, 1))
            .prop_flat_map(LinkedNode::arbitrary_with),
            BinaryOp::arbitrary()
                .prop_map(|v| Node::Expression(Expression::BinaryOp(v)))
                .prop_map(move |n| (n, 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            prop::strategy::Union::new(vec![
                FunctionCall::arbitrary_with(PROPTEST_DEEP_FACTOR + 1)
                    .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
            ])
            .prop_map(move |n| (n, 1))
            .prop_flat_map(LinkedNode::arbitrary_with),
        )
            .prop_map(move |(left, operator, right)| BinaryExp {
                left: Box::new(left),
                operator: Box::new(operator),
                right: Box::new(right),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
