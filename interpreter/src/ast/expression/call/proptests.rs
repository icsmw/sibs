use crate::*;

use lexer::{Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Call {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        FunctionCall::arbitrary_with(0)
            .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
            .boxed()
            .prop_map(move |node| Call {
                token: Token::for_test(Kind::Dot),
                node: Box::new(node),
            })
            .boxed()
    }
}

test_node_reading!(call, Call, 10);
