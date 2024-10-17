use crate::{
    elements::{Element, ElementId, Values},
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
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                } else {
                    vec![
                        ElementId::Command,
                        ElementId::Function,
                        ElementId::PatternString,
                        ElementId::Reference,
                        ElementId::Values,
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
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
