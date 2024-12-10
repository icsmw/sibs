use crate::*;
use proptest::prelude::*;

impl Arbitrary for VariableDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            VariableName::arbitrary()
                .prop_map(Declaration::VariableName)
                .prop_map(Node::Declaration)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            VariableTypeDeclaration::arbitrary_with(deep + 1)
                .prop_map(Declaration::VariableTypeDeclaration)
                .prop_map(Node::Declaration)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            AssignedValue::arbitrary_with(deep + 1)
                .prop_map(Statement::AssignedValue)
                .prop_map(Node::Statement)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            prop::strategy::Union::new(vec![Just(true), Just(false)]),
            prop::strategy::Union::new(vec![Just(true), Just(false)]),
        )
            .prop_map(
                |(variable, ty, assig, use_ty, use_assig)| VariableDeclaration {
                    variable: Box::new(variable),
                    r#type: if use_ty { Some(Box::new(ty)) } else { None },
                    assignation: if use_assig {
                        Some(Box::new(assig))
                    } else {
                        None
                    },
                    token: Token::for_test(Kind::Keyword(Keyword::Let)),
                    uuid: Uuid::new_v4(),
                },
            )
            .boxed()
    }
}
