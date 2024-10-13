use crate::{
    elements::{Cmp, Comparing, Element, ElementId},
    inf::tests::MAX_DEEP,
};
use proptest::prelude::*;

impl Arbitrary for Cmp {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Cmp::Equal),
            Just(Cmp::NotEqual),
            Just(Cmp::LeftBig),
            Just(Cmp::RightBig),
            Just(Cmp::LeftBigInc),
            Just(Cmp::RightBigInc)
        ]
        .boxed()
    }
}

impl Arbitrary for Comparing {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                } else {
                    vec![
                        ElementId::VariableName,
                        ElementId::Function,
                        ElementId::PatternString,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                },
                deep,
            )),
            Cmp::arbitrary(),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementId::VariableName,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                } else {
                    vec![
                        ElementId::VariableName,
                        ElementId::Function,
                        ElementId::PatternString,
                        ElementId::Integer,
                        ElementId::Boolean,
                    ]
                },
                deep,
            )),
        )
            .prop_map(|(left, cmp, right)| Comparing {
                cmp,
                left: Box::new(left),
                right: Box::new(right),
                token: 0,
            })
            .boxed()
    }
}
