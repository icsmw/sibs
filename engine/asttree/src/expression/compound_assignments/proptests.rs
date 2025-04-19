use crate::*;

use proptest::prelude::*;

impl Arbitrary for CompoundAssignments {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            CompoundAssignmentsOp::arbitrary()
                .prop_map(|v| Node::Expression(Expression::CompoundAssignmentsOp(v)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            if deep > PROPTEST_DEEP_FACTOR {
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                ])
            } else {
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    BinaryExpSeq::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                        .boxed(),
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                ])
            }
            .prop_map(move |n| (n, deep + 1))
            .prop_flat_map(LinkedNode::arbitrary_with),
        )
            .prop_map(move |(left, operator, right)| CompoundAssignments {
                left: Box::new(left),
                operator: Box::new(operator),
                right: Box::new(right),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
