use crate::*;

use lexer::{gens::kind, Kind, KindId};
use proptest::prelude::*;

impl Arbitrary for Variable {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        kind(KindId::Identifier)
            .prop_filter_map("single ident", |knds| {
                if let Some(Kind::Identifier(ident)) = knds.get(0) {
                    Some(ident.to_owned())
                } else {
                    None
                }
            })
            .prop_map(|ident| Variable { ident })
            .boxed()
    }
}
