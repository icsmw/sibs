use crate::elements::Boolean;
use proptest::prelude::*;

impl Arbitrary for Boolean {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![Just(true), Just(false)]
            .prop_map(|value| Boolean { value, token: 0 })
            .boxed()
    }
}
