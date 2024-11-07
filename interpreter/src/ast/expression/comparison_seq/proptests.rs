use crate::*;

use proptest::prelude::*;

impl Arbitrary for ComparisonSeq {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Comparison::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Comparison(v)))
                        .boxed(),
                    ComparisonGroup::arbitrary()
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
