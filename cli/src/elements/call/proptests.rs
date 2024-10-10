use crate::elements::{Call, Element, ElementRef};
use proptest::prelude::*;

impl Arbitrary for Call {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Element::arbitrary_with((vec![ElementRef::Function], deep))
            .prop_map(|el| Call {
                func: Box::new(el),
                token: 0,
            })
            .boxed()
    }
}
