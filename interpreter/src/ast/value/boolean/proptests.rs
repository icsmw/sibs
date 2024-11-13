use crate::*;

use lexer::{gens, Keyword, KeywordId, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Boolean {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::rnd_keyword_with(vec![KeywordId::True, KeywordId::False])
            .prop_filter_map(
                "Expected: KeywordId::True, KeywordId::False",
                |knd| match knd {
                    Kind::Keyword(Keyword::True) => Some(Boolean {
                        inner: true,
                        token: Token::for_test(Kind::Keyword(Keyword::True)),
                    }),
                    Kind::Keyword(Keyword::False) => Some(Boolean {
                        inner: false,
                        token: Token::for_test(Kind::Keyword(Keyword::False)),
                    }),
                    _ => None,
                },
            )
            .boxed()
    }
}

test_node_reading!(boolean, Boolean, 10);
