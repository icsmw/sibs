use crate::elements::{Condition, Element, ElementId};
use proptest::prelude::*;

impl Arbitrary for Condition {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Element::arbitrary_with((vec![ElementId::Subsequence], deep))
            .prop_map(|subsequence| Condition {
                subsequence: Box::new(subsequence),
                token: 0,
            })
            .boxed()
    }
}
