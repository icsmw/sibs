use crate::*;
use lexer::{gens, Keyword, KeywordId, Kind, Token};
use proptest::prelude::*;

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
                VariableCompoundTypeId::Vec => VariableCompoundType::Vec(
                    Token::for_test(Kind::Keyword(Keyword::Vec)),
                    Box::new(ty),
                ),
            })
            .boxed()
    }
}

impl Arbitrary for VariableType {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > 5 {
            prop::strategy::Union::new(vec![
                gens::keyword(KeywordId::Str),
                gens::keyword(KeywordId::Bool),
                gens::keyword(KeywordId::Num),
            ])
            .prop_map(|kw| VariableTypeDef::Primitive(Token::for_test(Kind::Keyword(kw))))
            .boxed()
        } else {
            prop::strategy::Union::new(vec![VariableCompoundType::arbitrary_with(deep + 1)
                .prop_map(VariableTypeDef::Compound)
                .boxed()])
            .boxed()
        }
        .prop_map(|ty| VariableType { r#type: ty })
        .boxed()
    }
}

test_node_reading!(variable_type, VariableType, 10);
