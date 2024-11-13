use crate::*;
use lexer::{Keyword, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for VariableDeclaration {
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
            AssignedValue::arbitrary_with(deep + 1)
                .prop_map(Statement::AssignedValue)
                .prop_map(Node::Statement)
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
                },
            )
            .boxed()
    }
}

test_node_reading!(variable_declaration, VariableDeclaration, 10);
