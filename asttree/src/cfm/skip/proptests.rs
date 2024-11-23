use crate::*;
use proptest::prelude::*;

impl Arbitrary for SkipTaskArgument {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        prop::strategy::Union::new(vec![
            Boolean::arbitrary()
                .prop_map(Value::Boolean)
                .prop_map(Node::Value)
                .boxed()
                .prop_map(SkipTaskArgument::Value)
                .boxed(),
            Number::arbitrary()
                .prop_map(Value::Number)
                .prop_map(Node::Value)
                .boxed()
                .prop_map(SkipTaskArgument::Value)
                .boxed(),
            PrimitiveString::arbitrary()
                .prop_map(Value::PrimitiveString)
                .prop_map(Node::Value)
                .boxed()
                .prop_map(SkipTaskArgument::Value)
                .boxed(),
            Array::arbitrary_with(deep + 1)
                .prop_map(Value::Array)
                .prop_map(Node::Value)
                .boxed()
                .prop_map(SkipTaskArgument::Value)
                .boxed(),
            Just(SkipTaskArgument::Any).boxed(),
        ])
        .boxed()
    }
}

impl Arbitrary for Skip {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(SkipTaskArgument::arbitrary_with(deep + 1).boxed(), 0..5),
            FunctionCall::arbitrary_with(deep + 1)
                .prop_map(Expression::FunctionCall)
                .prop_map(Node::Expression)
                .boxed(),
        )
            .prop_map(|(args, func)| Skip {
                token: Token::for_test(Kind::Identifier(String::from("skip"))),
                func: Box::new(func),
                args,
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
            })
            .boxed()
    }
}
