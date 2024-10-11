use crate::elements::Breaker;
use proptest::prelude::*;

impl Arbitrary for Breaker {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_deep: Self::Parameters) -> Self::Strategy {
        Just(Breaker { token: 0 }).boxed()
    }
}
