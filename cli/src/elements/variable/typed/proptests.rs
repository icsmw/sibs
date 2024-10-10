use crate::elements::{Types, VariableType};
use proptest::prelude::*;

impl Arbitrary for Types {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![Just(Types::String), Just(Types::Bool), Just(Types::Number),].boxed()
    }
}

impl Arbitrary for VariableType {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        Types::arbitrary()
            .prop_map(|var_type| VariableType { var_type, token: 0 })
            .boxed()
    }
}
