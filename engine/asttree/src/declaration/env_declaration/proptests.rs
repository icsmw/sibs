use crate::*;
use proptest::prelude::*;

impl Arbitrary for EnvDeclaration {
    type Parameters = u8;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_deep: Self::Parameters) -> Self::Strategy {
        (
            prop_oneof![Just(Keyword::Required), Just(Keyword::Optional)],
            VariableName::arbitrary(),
        )
            .prop_map(|(keyword, variable)| EnvDeclaration {
                token: Token::for_test(Kind::Keyword(keyword)),
                variable: Box::new(LinkedNode::from_node(variable.into())),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
