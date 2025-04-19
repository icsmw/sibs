use crate::*;
use proptest::prelude::*;

impl Arbitrary for Variable {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            gens::kind(KindId::Identifier),
            prop::strategy::Union::new(vec![Just(Some(Token::for_test(Kind::Bang))), Just(None)]),
        )
            .prop_filter_map("Expected: Kind::Identifier", |(knd, negation)| {
                if let Kind::Identifier(ident) = knd {
                    Some(Variable {
                        ident: ident.to_owned(),
                        negation,
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
