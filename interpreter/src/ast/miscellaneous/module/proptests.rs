use crate::*;
use lexer::{Keyword, Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Module {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        PrimitiveString::arbitrary()
            .prop_map(|node| Module {
                token: Token::for_test(Kind::Keyword(Keyword::Mod)),
                node: Box::new(Node::Value(Value::PrimitiveString(node))),
            })
            .boxed()
    }
}

test_node_reading!(module, Module, 10);
