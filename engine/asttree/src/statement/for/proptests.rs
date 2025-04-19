use crate::*;
use proptest::prelude::*;

impl Arbitrary for For {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            Variable::arbitrary()
                .prop_map(|n| Node::Expression(Expression::Variable(n)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
            prop::option::of(
                Variable::arbitrary()
                    .prop_map(|n| Node::Expression(Expression::Variable(n)))
                    .prop_map(move |n| (n, deep + 1))
                    .prop_flat_map(LinkedNode::arbitrary_with)
                    .boxed(),
            ),
            if deep > PROPTEST_DEEP_FACTOR {
                prop::strategy::Union::new(vec![
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Range::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Range(v)))
                        .boxed(),
                ])
            } else {
                prop::strategy::Union::new(vec![
                    Array::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::Array(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Range::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Range(v)))
                        .boxed(),
                ])
            }
            .prop_map(move |n| (n, deep + 1))
            .prop_flat_map(LinkedNode::arbitrary_with),
            Block::arbitrary_with(deep + 1)
                .prop_map(|n| Node::Statement(Statement::Block(n)))
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(move |(element, index, elements, block)| For {
                element: Box::new(element),
                index: index.map(Box::new),
                elements: Box::new(elements),
                block: Box::new(block),
                token_for: Token::for_test(Kind::Keyword(Keyword::For)),
                token_in: Token::for_test(Kind::Keyword(Keyword::In)),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
