use crate::{
    elements::{Cmp, Comparing, Element, ElementRef},
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
                        ElementRef::VariableName,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                } else {
                    vec![
                        ElementRef::VariableName,
                        ElementRef::Function,
                        ElementRef::PatternString,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                },
                deep,
            )),
            Cmp::arbitrary(),
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementRef::VariableName,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                } else {
                    vec![
                        ElementRef::VariableName,
                        ElementRef::Function,
                        ElementRef::PatternString,
                        ElementRef::Integer,
                        ElementRef::Boolean,
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
