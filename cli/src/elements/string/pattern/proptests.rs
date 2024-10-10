use crate::{
    elements::{Element, ElementRef, Metadata, PatternString, SimpleString},
    inf::tests::MAX_DEEP,
};
use proptest::prelude::*;

impl Arbitrary for PatternString {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > MAX_DEEP {
            "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|pattern| {
                    let pattern = if pattern.len() < 3 {
                        "min".to_owned()
                    } else {
                        pattern
                    };
                    PatternString {
                        elements: vec![Element::SimpleString(
                            SimpleString {
                                value: pattern.clone(),
                                token: 0,
                            },
                            Metadata::empty(),
                        )],
                        token: 0,
                    }
                })
                .boxed()
        } else {
            (
                prop::collection::vec(
                    Element::arbitrary_with((
                        vec![
                            ElementRef::VariableName,
                            ElementRef::Function,
                            ElementRef::If,
                        ],
                        deep,
                    )),
                    0..=2,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementRef::SimpleString], deep)),
                    3,
                ),
            )
                .prop_map(|(injections, mut noise)| {
                    let mut elements: Vec<Element> = Vec::new();
                    for injection in injections.into_iter() {
                        elements.extend_from_slice(&[noise.remove(0), injection]);
                    }
                    elements.push(noise.remove(0));
                    PatternString { elements, token: 0 }
                })
                .boxed()
        }
    }
}
