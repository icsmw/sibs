use crate::elements::{Cmb, Combination};
use proptest::prelude::*;

impl Arbitrary for Combination {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Combination {
                cmb: Cmb::And,
                token: 0
            }),
            Just(Combination {
                cmb: Cmb::Or,
                token: 0
            }),
        ]
        .boxed()
    }
}
