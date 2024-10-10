use crate::{elements::variable::VariableName, reader::words};
use proptest::prelude::*;

impl Arbitrary for VariableName {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        "[a-z][a-z0-9]*"
            .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
            .prop_map(String::from)
            .prop_map(|name| VariableName {
                name: if name.is_empty() {
                    "min".to_owned()
                } else {
                    name
                },
                token: 0,
            })
            .boxed()
    }
}
