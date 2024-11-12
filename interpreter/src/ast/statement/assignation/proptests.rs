use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Assignation {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            AssignedValue::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Statement(Statement::AssignedValue(v)))
                .boxed(),
        )
            .prop_map(move |(left, right)| Assignation {
                left: Box::new(left),
                right: Box::new(right),
            })
            .boxed()
    }
}

test_node_reading!(assignation, Assignation, 10);
