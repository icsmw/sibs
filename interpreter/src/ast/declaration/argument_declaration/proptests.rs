use crate::*;
use lexer::{Keyword, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for ArgumentDeclaration {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(Expression::Variable)
                .prop_map(Node::Expression)
                .boxed(),
            VariableTypeDeclaration::arbitrary_with(deep + 1)
                .prop_map(Declaration::VariableTypeDeclaration)
                .prop_map(Node::Declaration)
                .boxed(),
        )
            .prop_map(|(variable, ty)| ArgumentDeclaration {
                variable: Box::new(variable),
                r#type: Box::new(ty),
            })
            .boxed()
    }
}

test_node_reading!(argument_declaration, ArgumentDeclaration, 10);
