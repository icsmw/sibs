use crate::*;

use lexer::{Keyword, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Loop {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        Block::arbitrary_with(deep + 1)
            .prop_map(|v| Node::Statement(Statement::Block(v)))
            .boxed()
            .prop_map(move |block| Loop {
                block: Box::new(block),
                token: Token::for_test(Kind::Keyword(Keyword::Loop)),
            })
            .boxed()
    }
}

test_node_reading!(r#loop, Loop, 10);
