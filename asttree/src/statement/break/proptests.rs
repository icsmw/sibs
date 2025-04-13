use crate::*;
use proptest::prelude::*;

impl Arbitrary for Break {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::keyword(KeywordId::Break)
            .boxed()
            .prop_map(move |knd| Break {
                token: Token::for_test(Kind::Keyword(knd)),
                target: None,
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
