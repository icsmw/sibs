use crate::*;
use proptest::prelude::*;

impl Arbitrary for RootMeta {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::kind(KindId::RootMeta)
            .prop_map(|kind| RootMeta {
                token: Token::for_test(kind),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
