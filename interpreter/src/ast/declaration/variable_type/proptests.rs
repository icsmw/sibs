use crate::*;
use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for VariablePrimitiveType {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(VariablePrimitiveType::String),
            Just(VariablePrimitiveType::Boolean),
            Just(VariablePrimitiveType::Number)
        ]
        .boxed()
    }
}

impl Arbitrary for VariableCompoundType {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            VariableType::arbitrary_with(deep + 1)
                .prop_map(|v| Node::Declaration(Declaration::VariableType(v)))
                .boxed(),
            prop::strategy::Union::new(vec![Just(VariableCompoundTypeId::Vec)]),
        )
            .prop_map(|(ty, id)| match id {
                VariableCompoundTypeId::Vec => VariableCompoundType::Vec(Box::new(ty)),
            })
            .boxed()
    }
}

impl Arbitrary for VariableType {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > 5 {
            prop::strategy::Union::new(vec![VariablePrimitiveType::arbitrary()
                .prop_map(VariableTypeDef::Primitive)
                .boxed()])
        } else {
            prop::strategy::Union::new(vec![VariableCompoundType::arbitrary_with(deep + 1)
                .prop_map(VariableTypeDef::Compound)
                .boxed()])
        }
        .prop_map(|ty| {
            let token = Token::for_test(Kind::Identifier(ty.to_ident()));
            VariableType { r#type: ty, token }
        })
        .boxed()
    }
}

test_node_reading!(variable_type, VariableType, 10);
