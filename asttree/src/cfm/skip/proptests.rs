use crate::*;
use proptest::prelude::*;

impl Arbitrary for Skip {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        (
            prop::collection::vec(
                ArgumentAssignation::arbitrary_with(deep + 1)
                    .prop_map(Statement::ArgumentAssignation)
                    .prop_map(Node::Statement)
                    // Take into account meta isn't included above ArgumentAssignation
                    .prop_map(LinkedNode::from_node)
                    .boxed(),
                0..5,
            ),
            FunctionCall::arbitrary_with(deep + 1)
                .prop_map(Expression::FunctionCall)
                .prop_map(Node::Expression)
                .prop_map(move |n| (n, deep + 1))
                .prop_flat_map(LinkedNode::arbitrary_with)
                .boxed(),
        )
            .prop_map(|(args, func)| Skip {
                token: Token::for_test(Kind::Identifier(String::from("skip"))),
                func: Box::new(func),
                args,
                open: Token::for_test(Kind::LeftParen),
                close: Token::for_test(Kind::RightParen),
                uuid: Uuid::new_v4(),
            })
            .boxed()
    }
}
