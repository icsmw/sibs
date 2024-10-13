use crate::elements::{Element, ElementId, VariableDeclaration};
use proptest::prelude::*;

impl Arbitrary for VariableDeclaration {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                vec![ElementId::VariableType, ElementId::VariableVariants],
                deep,
            )),
            Element::arbitrary_with((vec![ElementId::VariableName], deep)),
        )
            .prop_map(move |(declaration, variable)| VariableDeclaration {
                declaration: Box::new(declaration),
                variable: Box::new(variable),
                token: 0,
            })
            .boxed()
    }
}
