use crate::*;
use proptest::prelude::*;

impl Arbitrary for GlobalDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop_oneof![Just(Keyword::Global), Just(Keyword::Const)],
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
        )
            .prop_map(|(keyword, variable, ty, assig)| GlobalDeclaration {
                variable: Box::new(variable),
                r#type: Box::new(ty),
                assignation: Box::new(assig),
                token: Token::for_test(Kind::Keyword(keyword)),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
