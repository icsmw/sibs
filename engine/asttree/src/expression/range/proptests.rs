use crate::*;

use proptest::prelude::*;

impl Arbitrary for Range {
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
            ])
            .prop_map(move |n| (n, 1))
            .prop_flat_map(LinkedNode::arbitrary_with),
            prop::strategy::Union::new(vec![
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
            .prop_map(move |(left, right)| Range {
                left: Box::new(left),
                right: Box::new(right),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
