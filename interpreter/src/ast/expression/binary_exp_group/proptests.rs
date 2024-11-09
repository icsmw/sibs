use crate::*;

use proptest::prelude::*;

impl Arbitrary for BinaryExpGroup {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            if deep > 5 {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![BinaryExpPri::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::BinaryExpPri(v)))
                        .boxed()]),
                    1..5,
                )
            } else {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        BinaryExpPri::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::BinaryExpPri(v)))
                            .boxed(),
                        BinaryExpGroup::arbitrary_with(deep + 1)
                            .prop_map(|v| Node::Expression(Expression::BinaryExpGroup(v)))
                            .boxed(),
                    ]),
                    1..5,
                )
            },
            prop::collection::vec(
                BinaryOp::arbitrary_with(())
                    .prop_map(|v| Node::Expression(Expression::BinaryOp(v)))
                    .boxed(),
                5,
            ),
        )
            .prop_map(move |(comps, mut ops)| {
                let mut nodes = Vec::new();
                for comp in comps.into_iter() {
                    nodes.push(comp);
                    nodes.push(ops.remove(0));
                }
                nodes.remove(nodes.len() - 1);
                BinaryExpGroup { nodes }
            })
            .boxed()
    }
}

test_node_reading!(binary_exp_group, BinaryExpGroup, 10);
