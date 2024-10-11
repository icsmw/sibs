use crate::{
    elements::{Element, ElementRef, IfSubsequence},
    inf::tests::MAX_DEEP,
};
use proptest::prelude::*;

impl Arbitrary for IfSubsequence {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElementRef::Boolean,
                            ElementRef::Comparing,
                            ElementRef::Function,
                            ElementRef::VariableName,
                            ElementRef::Reference,
                        ]
                    } else {
                        vec![
                            ElementRef::Boolean,
                            ElementRef::Command,
                            ElementRef::Comparing,
                            ElementRef::Function,
                            ElementRef::VariableName,
                            ElementRef::Reference,
                            ElementRef::IfCondition,
                        ]
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
                IfSubsequence {
                    subsequence: result,
                    token: 0,
                }
            })
            .boxed()
    }
}
