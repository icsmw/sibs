use crate::*;
use proptest::prelude::*;

impl Arbitrary for ClosureDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                ArgumentDeclaration::arbitrary_with(deep + 1)
                    .prop_map(Declaration::ArgumentDeclaration)
                    .prop_map(Node::Declaration)
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
                1..5,
            ),
            VariableTypeDeclaration::arbitrary_with(deep + 1)
                .prop_map(Declaration::VariableTypeDeclaration)
                .prop_map(Node::Declaration)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(|(args, ty)| ClosureDeclaration {
                token: Token::for_test(Kind::Colon),
                ty: Box::new(ty),
                args,
                open: Token::for_test(Kind::VerticalBar),
                close: Token::for_test(Kind::VerticalBar),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
