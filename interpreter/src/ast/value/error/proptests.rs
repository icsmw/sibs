use crate::*;

use lexer::{ Kind, Token};
use proptest::prelude::*;

impl Arbitrary for Error {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop::strategy::Union::new(vec![
            Variable::arbitrary()
                .prop_map(|v| Node::Expression(Expression::Variable(v)))
                .boxed(),
            Number::arbitrary()
                .prop_map(|v| Node::Value(Value::Number(v)))
                .boxed(),
            PrimitiveString::arbitrary()
                .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                .boxed(),
        ])
        .prop_map(move |node| Error {
            node: Box::new(node),
            token: Token::for_test(Kind::Identifier(String::from("Error"))),
        })
        .boxed()
    }
}

test_node_reading!(error, Error, 10);
