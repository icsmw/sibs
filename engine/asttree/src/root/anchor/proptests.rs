use crate::*;
use proptest::prelude::*;

impl Arbitrary for Anchor {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            prop_oneof![
                Component::arbitrary_with(deep + 1)
                    .prop_map(Root::Component)
                    .prop_map(Node::Root)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                ModuleDeclaration::arbitrary()
                    .prop_map(Declaration::ModuleDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                IncludeDeclaration::arbitrary()
                    .prop_map(Declaration::IncludeDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                Module::arbitrary()
                    .prop_map(Root::Module)
                    .prop_map(Node::Root)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
            ],
            1..10,
        )
        .prop_map(|nodes| Anchor {
            nodes,
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
