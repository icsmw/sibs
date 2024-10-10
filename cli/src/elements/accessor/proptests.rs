use crate::elements::{Accessor, Element, ElementRef};
use proptest::prelude::*;

impl Arbitrary for Accessor {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Element::arbitrary_with((
            vec![
                ElementRef::Function,
                ElementRef::VariableName,
                ElementRef::Integer,
            ],
            deep,
        ))
        .prop_map(|el| Accessor {
            index: Box::new(el),
            token: 0,
        })
        .boxed()
    }
}
