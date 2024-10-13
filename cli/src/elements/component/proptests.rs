use std::path::PathBuf;

use crate::elements::{Component, Element, ElementId, SimpleString};
use proptest::prelude::*;
use uuid::Uuid;

impl Arbitrary for Component {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            "[a-zA-Z]*".prop_map(String::from),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementId::Task], deep)),
                2..6,
            ),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementId::Meta], deep)),
                0..3,
            ),
            prop::collection::vec(
                Element::arbitrary_with((vec![ElementId::Function], deep)),
                0..3,
            ),
        )
            .prop_map(|(name, tasks, meta, funcs)| Component {
                uuid: Uuid::new_v4(),
                elements: [meta, funcs, tasks].concat(),
                name: SimpleString {
                    value: name,
                    token: 0,
                },
                cwd: Some(PathBuf::new()),
                token: 0,
            })
            .boxed()
    }
}
