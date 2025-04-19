use crate::*;

use proptest::prelude::*;

impl Arbitrary for ComparisonSeq {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            if deep > PROPTEST_DEEP_FACTOR {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        Variable::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::Variable(v)))
                            .boxed(),
                        Comparison::arbitrary_with(deep + 1)
                            .prop_map(|v| Node::Expression(Expression::Comparison(v)))
                            .boxed(),
                    ])
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with),
                    1..5,
                )
                .boxed()
            } else {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        Variable::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::Variable(v)))
                            .boxed(),
                        FunctionCall::arbitrary_with(PROPTEST_DEEP_FACTOR + 1)
                            .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                            .boxed(),
                        Comparison::arbitrary_with(deep + 1)
                            .prop_map(|v| Node::Expression(Expression::Comparison(v)))
                            .boxed(),
                        ComparisonGroup::arbitrary_with(deep + 1)
                            .prop_map(|v| Node::Expression(Expression::ComparisonGroup(v)))
                            .boxed(),
                    ])
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with),
                    1..5,
                )
                .boxed()
            },
            prop::collection::vec(
                LogicalOp::arbitrary_with(())
                    .prop_map(|v| Node::Expression(Expression::LogicalOp(v)))
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
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
                ComparisonSeq {
                    nodes,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
