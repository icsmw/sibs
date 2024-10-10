use crate::{
    elements::{Conclusion, Element, ElementRef},
    inf::tests::MAX_DEEP,
};
use proptest::prelude::*;

impl Arbitrary for Conclusion {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElementRef::Comparing]
                    } else {
                        vec![ElementRef::Comparing, ElementRef::Condition]
                    },
                    deep,
                )),
                1..=5,
            ),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementRef::Combination], deep)),
                5..=5,
            ),
        )
            .prop_map(|(mut subsequences, mut combinations)| {
                let mut result: Vec<Element> = Vec::new();
                while let Some(subsequence) = subsequences.pop() {
                    result.push(subsequence);
                    result.push(combinations.pop().unwrap());
                }
                let _ = result.pop();
                Conclusion {
                    subsequence: result,
                    token: 0,
                }
            })
            .boxed()
    }
}
