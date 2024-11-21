use crate::*;

use lexer::{gens::kind, Kind, KindId, Token};
use proptest::prelude::*;

impl Arbitrary for Variable {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        kind(KindId::Identifier)
            .prop_filter_map("Expected: Kind::Identifier", |knd| {
                if let Kind::Identifier(ident) = knd {
                    Some(Variable {
                        ident: ident.to_owned(),
                        token: Token::for_test(Kind::Identifier(ident.to_owned())),
                    })
                } else {
                    None
                }
            })
            .boxed()
    }
}
