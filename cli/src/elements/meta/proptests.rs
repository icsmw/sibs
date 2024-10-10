use crate::elements::meta::Meta;
use proptest::prelude::*;

impl Arbitrary for Meta {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 1..=10)
            .prop_map(|inner| Meta { inner, token: 0 })
            .boxed()
    }
}
