use crate::*;
use proptest::prelude::*;

impl Arbitrary for Number {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::Number)
            .prop_filter_map("Expected: Kind::Number", |knd| {
                if let Kind::Number(n) = knd {
                    Some(Number {
                        inner: n.to_owned(),
                        token: Token::for_test(Kind::Number(n.to_owned())),
                    })
                } else {
                    None
                }
            })
            .boxed()
    }
}
