use crate::elements::{Element, ElementRef, VariableDeclaration};
use proptest::prelude::*;

impl Arbitrary for VariableDeclaration {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                vec![ElementRef::VariableType, ElementRef::VariableVariants],
                deep,
            )),
            Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
        )
            .prop_map(move |(declaration, variable)| VariableDeclaration {
                declaration: Box::new(declaration),
                variable: Box::new(variable),
                token: 0,
            })
            .boxed()
    }
}
