use crate::*;
use proptest::prelude::*;

impl Arbitrary for ArgumentAssignation {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            ArgumentAssignedValue::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Statement(Statement::ArgumentAssignedValue(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(move |(left, right)| ArgumentAssignation {
                left: Box::new(left),
                right: Box::new(right),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
