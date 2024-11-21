use crate::*;

use proptest::prelude::*;

impl Arbitrary for BinaryExp {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            prop::strategy::Union::new(vec![
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
            ]),
            BinaryOp::arbitrary()
                .prop_map(|v| Node::Expression(Expression::BinaryOp(v)))
                .boxed(),
            prop::strategy::Union::new(vec![
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
            ]),
        )
            .prop_map(move |(left, operator, right)| BinaryExp {
                left: Box::new(left),
                operator: Box::new(operator),
                right: Box::new(right),
            })
            .boxed()
    }
}
