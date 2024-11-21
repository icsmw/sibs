use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Call {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        FunctionCall::arbitrary_with(deep + 1)
            .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
            .boxed()
            .prop_map(move |node| Call {
                token: Token::for_test(Kind::Dot),
                node: Box::new(node),
            })
            .boxed()
    }
}
