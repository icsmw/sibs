use crate::*;

use proptest::prelude::*;

impl Arbitrary for BinaryExpSeq {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            if deep > PROPTEST_DEEP_FACTOR {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        Number::arbitrary()
                            .prop_map(|v| Node::Value(Value::Number(v)))
                            .boxed(),
                        Variable::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::Variable(v)))
                            .boxed(),
                        BinaryExp::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::BinaryExp(v)))
                            .boxed(),
                    ])
                    .prop_flat_map(LinkedNode::arbitrary_with),
                    1..5,
                )
            } else {
                prop::collection::vec(
                    prop::strategy::Union::new(vec![
                        Number::arbitrary()
                            .prop_map(|v| Node::Value(Value::Number(v)))
                            .boxed(),
                        BinaryExp::arbitrary()
                            .prop_map(|v| Node::Expression(Expression::BinaryExp(v)))
                            .boxed(),
                        BinaryExpGroup::arbitrary_with(deep + 1)
                            .prop_map(|v| Node::Expression(Expression::BinaryExpGroup(v)))
                            .boxed(),
                    ])
                    .prop_flat_map(LinkedNode::arbitrary_with),
                    1..5,
                )
            },
            prop::collection::vec(
                BinaryOp::arbitrary_with(())
                    .prop_map(|v| Node::Expression(Expression::BinaryOp(v)))
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
                BinaryExpSeq {
                    nodes,
                    uuid: Uuid::new_v4(),
                }
            })
            .boxed()
    }
}
