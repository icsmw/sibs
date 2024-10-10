use crate::{elements::VariableVariants, inf::Value, reader::words};
use proptest::prelude::*;

impl Arbitrary for VariableVariants {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::collection::vec(
            "[a-z][a-z0-9]*"
                .prop_filter("exclude keywords", move |s: &String| !words::is_reserved(s))
                .prop_map(String::from),
            2..=10,
        )
        .prop_map(|values| VariableVariants {
            values: values
                .iter()
                .map(|v| {
                    Value::String(if v.is_empty() {
                        "min".to_owned()
                    } else {
                        v.to_owned()
                    })
                })
                .collect::<Vec<Value>>(),
            token: 0,
        })
        .boxed()
    }
}
