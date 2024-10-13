use crate::{
    elements::{Element, ElementId, IfSubsequence},
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
                            ElementId::Boolean,
                            ElementId::Comparing,
                            ElementId::Function,
                            ElementId::VariableName,
                            ElementId::Reference,
                        ]
                    } else {
                        vec![
                            ElementId::Boolean,
                            ElementId::Command,
                            ElementId::Comparing,
                            ElementId::Function,
                            ElementId::VariableName,
                            ElementId::Reference,
                            ElementId::IfCondition,
                        ]
                    },
                    deep,
                )),
                1..=5,
            ),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementId::Combination], deep)),
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
