use crate::*;

use proptest::prelude::*;

impl Arbitrary for Block {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_deep: Self::Parameters) -> Self::Strategy {
        Just(true).prop_map(|_| Block { nodes: vec![] }).boxed()
    }
}
