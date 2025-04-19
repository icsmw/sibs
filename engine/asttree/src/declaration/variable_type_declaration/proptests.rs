use crate::*;
use proptest::prelude::*;

impl Arbitrary for VariableTypeDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            VariableType::arbitrary_with(deep + 1)
                .prop_map(Declaration::VariableType)
                .prop_map(Node::Declaration)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            1..5,
        )
        .prop_map(|types| {
            let token = Token::for_test(Kind::Colon);
            VariableTypeDeclaration {
                types,
                token,
                uuid: Uuid::new_v4(),
            }
        })
        .boxed()
    }
}
