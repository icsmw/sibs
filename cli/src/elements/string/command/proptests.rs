use crate::{
    elements::{Command, Element, ElementId, Metadata, SimpleString},
    inf::tests::*,
};
use proptest::prelude::*;

impl Arbitrary for Command {
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
                    Command {
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
                            ElementId::VariableName,
                            ElementId::Function,
                            ElementId::If,
                        ],
                        deep,
                    )),
                    0..=2,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementId::SimpleString], deep)),
                    3,
                ),
            )
                .prop_map(|(injections, mut noise)| {
                    let mut elements: Vec<Element> = Vec::new();
                    for injection in injections.into_iter() {
                        elements.extend_from_slice(&[noise.remove(0), injection]);
                    }
                    elements.push(noise.remove(0));
                    Command { elements, token: 0 }
                })
                .boxed()
        }
    }
}
