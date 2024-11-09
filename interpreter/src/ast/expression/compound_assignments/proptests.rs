use crate::*;

use proptest::prelude::*;

impl Arbitrary for CompoundAssignments {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            CompoundAssignmentsOp::arbitrary()
                .prop_map(|v| Node::Expression(Expression::CompoundAssignmentsOp(v)))
                .boxed(),
            prop::strategy::Union::new(vec![
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                BinaryExpSeq::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                    .boxed(),
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
            ]),
        )
            .prop_map(move |(left, operator, right)| CompoundAssignments {
                left: Box::new(left),
                operator: Box::new(operator),
                right: Box::new(right),
            })
            .boxed()
    }
}

test_node_reading!(compound_assignments, CompoundAssignments, 10);
