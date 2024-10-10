use crate::elements::Integer;
use proptest::prelude::*;

impl Arbitrary for Integer {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (isize::MIN..isize::MAX)
            .prop_map(|value| Integer { value, token: 0 })
            .boxed()
    }
}
