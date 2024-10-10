use crate::{
    elements::{Element, ElementRef, Values},
    inf::tests::*,
};
use proptest::prelude::*;

impl Arbitrary for Values {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        let max = 5;
        prop::collection::vec(
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementRef::VariableName,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                } else {
                    vec![
                        ElementRef::Command,
                        ElementRef::Function,
                        ElementRef::If,
                        ElementRef::PatternString,
                        ElementRef::Reference,
                        ElementRef::Values,
                        ElementRef::Comparing,
                        ElementRef::VariableName,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                },
                deep,
            )),
            1..max,
        )
        .prop_map(|elements| Values { elements, token: 0 })
        .boxed()
    }
}
