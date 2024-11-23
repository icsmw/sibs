use crate::*;
use proptest::prelude::*;

impl Arbitrary for PrimitiveString {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::String)
            .prop_filter_map("Expected: Kind::String", |knd| {
                if let Kind::String(inner) = knd.clone() {
                    Some(PrimitiveString {
                        inner,
                        token: Token::for_test(knd),
                    })
                } else {
                    None
                }
            })
            .boxed()
    }
}
