use crate::elements::Comment;
use proptest::prelude::*;

impl Arbitrary for Comment {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        "[a-z][a-z0-9]*"
            .prop_map(String::from)
            .prop_map(|comment| Comment { comment, token: 0 })
            .boxed()
    }
}
