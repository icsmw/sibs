use crate::*;

use lexer::{gens::kind, Kind, KindId, Token};
use prop::test_runner::Reason;
use proptest::prelude::*;

impl Arbitrary for Variable {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        kind(KindId::Identifier)
            .prop_filter_map("Expected: Kind::Identifier", |knds| {
                if let Some(Kind::Identifier(ident)) = knds.first() {
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

test_node_reading!(мariable, Variable, 10);
