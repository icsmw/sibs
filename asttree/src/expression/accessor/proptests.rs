use crate::*;
use proptest::prelude::*;

impl Arbitrary for Accessor {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > PROPTEST_DEEP_FACTOR {
            prop::strategy::Union::new(vec![
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
            ])
        } else {
            prop::strategy::Union::new(vec![
                Number::arbitrary()
                    .prop_map(|v| Node::Value(Value::Number(v)))
                    .boxed(),
                FunctionCall::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                    .boxed(),
                Variable::arbitrary()
                    .prop_map(|v| Node::Expression(Expression::Variable(v)))
                    .boxed(),
                BinaryExpSeq::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Expression(Expression::BinaryExpSeq(v)))
                    .boxed(),
                If::arbitrary_with(deep + 1)
                    .prop_map(|v| Node::Statement(Statement::If(v)))
                    .boxed(),
            ])
        }
        .prop_flat_map(LinkedNode::arbitrary_with)
        .prop_map(move |node| Accessor {
            node: Box::new(node),
            open: Token::for_test(Kind::LeftBracket),
            close: Token::for_test(Kind::RightBracket),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
