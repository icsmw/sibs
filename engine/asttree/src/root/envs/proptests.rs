use crate::*;
use proptest::prelude::*;

impl Arbitrary for EnvsModule {
    type Parameters = u8;
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(_deep: u8) -> Self::Strategy {
        prop::collection::vec(
            prop_oneof![
                Comment::arbitrary().prop_map(|n| LinkedNode::from_node(n.into())),
                EnvDeclaration::arbitrary().prop_map(|n| LinkedNode::from_node(n.into()))
            ],
            0..5,
        )
        .prop_map(|nodes| EnvsModule {
            nodes,
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
