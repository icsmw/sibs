use crate::elements::{Element, ElementId, Range};
use proptest::prelude::*;

impl Arbitrary for Range {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;
    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((vec![ElementId::VariableName, ElementId::Integer], deep)),
            Element::arbitrary_with((vec![ElementId::VariableName, ElementId::Integer], deep)),
        )
            .prop_map(|(from, to)| Range {
                from: Box::new(from),
                to: Box::new(to),
                token: 0,
            })
            .boxed()
    }
}
