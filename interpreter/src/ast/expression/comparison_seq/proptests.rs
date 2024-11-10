use crate::*;

use proptest::prelude::*;

impl Arbitrary for ComparisonSeq {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Comparison::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::Comparison(v)))
                        .boxed(),
                    ComparisonGroup::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::ComparisonGroup(v)))
                        .boxed(),
                ]),
                1..5,
            ),
            prop::collection::vec(
                LogicalOp::arbitrary_with(())
                    .prop_map(|v| Node::Expression(Expression::LogicalOp(v)))
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
                ComparisonSeq { nodes }
            })
            .boxed()
    }
}

test_node_reading!(comparison_seq, ComparisonSeq, 10);
