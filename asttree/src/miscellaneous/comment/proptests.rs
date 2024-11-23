use crate::*;
use proptest::prelude::*;

impl Arbitrary for Comment {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::Comment)
            .prop_map(|kind| Comment {
                token: Token::for_test(kind),
            })
            .boxed()
    }
}
