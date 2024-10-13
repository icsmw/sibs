use crate::{
    elements::{Element, ElementId, Reference},
    inf::tests::*,
    reader::words,
};
use proptest::prelude::*;

impl Arbitrary for Reference {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > MAX_DEEP {
            prop::collection::vec(
                "[a-z][a-z0-9]*"
                    .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                    .prop_map(String::from),
                2,
            )
            .prop_map(|path| Reference {
                path: path
                    .iter()
                    .map(|p| {
                        if p.is_empty() {
                            "min".to_owned()
                        } else {
                            p.to_owned()
                        }
                    })
                    .collect::<Vec<String>>(),
                inputs: Vec::new(),
                token: 0,
            })
            .boxed()
        } else {
            (
                prop::collection::vec(
                    "[a-z][a-z0-9]*"
                        .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                        .prop_map(String::from),
                    2,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((
                        vec![
                            ElementId::VariableName,
                            ElementId::Integer,
                            ElementId::Boolean,
                            ElementId::PatternString,
                        ],
                        deep,
                    )),
                    0..5,
                ),
            )
                .prop_map(|(path, inputs)| Reference {
                    path: path
                        .iter()
                        .map(|p| {
                            if p.is_empty() {
                                "min".to_owned()
                            } else {
                                p.to_owned()
                            }
                        })
                        .collect::<Vec<String>>(),
                    inputs,
                    token: 0,
                })
                .boxed()
        }
    }
}
