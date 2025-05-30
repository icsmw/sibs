use crate::*;
use proptest::prelude::*;

impl Arbitrary for Gatekeeper {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            prop::strategy::Union::new(vec![Skip::arbitrary_with(deep + 1)
                .prop_map(ControlFlowModifier::Skip)
                .prop_map(Node::ControlFlowModifier)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed()]),
            1..5,
        )
        .prop_map(|nodes| Gatekeeper {
            token: Token::for_test(Kind::Pound),
            nodes,
            open: Token::for_test(Kind::LeftBracket),
            close: Token::for_test(Kind::RightBracket),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
