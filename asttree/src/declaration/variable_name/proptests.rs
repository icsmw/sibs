use crate::*;
use proptest::prelude::*;

impl Arbitrary for VariableName {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::Identifier)
            .prop_filter_map("Expected: Kind::Identifier", |knd| {
                if let Kind::Identifier(ident) = knd {
                    Some(VariableName {
                        ident: ident.to_owned(),
                        token: Token::for_test(Kind::Identifier(ident.to_owned())),
                        uuid: Uuid::new_v4(),
                    })
                } else {
                    None
                }
            })
            .boxed()
    }
}
