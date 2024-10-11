use crate::elements::{Element, ElementRef, IfCondition};
use proptest::prelude::*;

impl Arbitrary for IfCondition {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Element::arbitrary_with((vec![ElementRef::IfSubsequence], deep))
            .prop_map(|subsequence| IfCondition {
                subsequence: Box::new(subsequence),
                token: 0,
            })
            .boxed()
    }
}
