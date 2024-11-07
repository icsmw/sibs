use crate::*;

use proptest::prelude::*;

impl Arbitrary for Block {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        Just(true).prop_map(|_| Block { nodes: vec![] }).boxed()
    }
}
