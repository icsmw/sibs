use crate::*;
use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Gatekeeper {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            prop::strategy::Union::new(vec![Skip::arbitrary_with(deep + 1)
                .prop_map(ControlFlowModifier::Skip)
                .prop_map(Node::ControlFlowModifier)
                .boxed()]),
            1..5,
        )
        .prop_map(|nodes| Gatekeeper {
            token: Token::for_test(Kind::Pound),
            nodes,
        })
        .boxed()
    }
}

test_node_reading!(gatekeeper, Gatekeeper, 10);

// test_node_reading_case!(gatekeeper_case, Gatekeeper, "#[skip([*,2], func())]");
