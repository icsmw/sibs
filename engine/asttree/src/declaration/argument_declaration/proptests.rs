use crate::*;
use proptest::prelude::*;

impl Arbitrary for ArgumentDeclaration {
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
            if deep > PROPTEST_DEEP_FACTOR {
                prop::strategy::Union::new(vec![
                    VariableTypeDeclaration::arbitrary_with(deep + 1)
                        .prop_map(Declaration::VariableTypeDeclaration)
                        .prop_map(Node::Declaration)
                        .boxed(),
                    VariableVariants::arbitrary()
                        .prop_map(Declaration::VariableVariants)
                        .prop_map(Node::Declaration)
                        .boxed(),
                ])
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed()
            } else {
                prop::strategy::Union::new(vec![
                    VariableTypeDeclaration::arbitrary_with(deep + 1)
                        .prop_map(Declaration::VariableTypeDeclaration)
                        .prop_map(Node::Declaration)
                        .boxed(),
                    VariableVariants::arbitrary()
                        .prop_map(Declaration::VariableVariants)
                        .prop_map(Node::Declaration)
                        .boxed(),
                    ClosureDeclaration::arbitrary_with(deep + 1)
                        .prop_map(Declaration::ClosureDeclaration)
                        .prop_map(Node::Declaration)
                        .boxed(),
                ])
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed()
            },
        )
            .prop_map(|(variable, ty)| ArgumentDeclaration {
                variable: Box::new(variable),
                r#type: Box::new(ty),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
