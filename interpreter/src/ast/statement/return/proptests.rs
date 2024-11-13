use crate::*;

use lexer::{gens, KeywordId, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Return {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        gens::keyword(KeywordId::Return)
            .boxed()
            .prop_map(move |knd| Return {
                token: Token::for_test(Kind::Keyword(knd)),
            })
            .boxed()
    }
}

test_node_reading!(r#return, Return, 10);
