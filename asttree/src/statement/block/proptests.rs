use crate::*;
use proptest::prelude::*;

impl Arbitrary for Block {
    type Parameters = u8;

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
        if deep > PROPTEST_DEEP_FACTOR {
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                    Command::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::Command(v)))
                        .boxed(),
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    TaskCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::TaskCall(v)))
                        .boxed(),
                    CompoundAssignments::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::CompoundAssignments(v)))
                        .boxed(),
                    Assignation::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Assignation(v)))
                        .boxed(),
                    Break::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Break(v)))
                        .boxed(),
                    VariableDeclaration::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Declaration(Declaration::VariableDeclaration(v)))
                        .boxed(),
                ])
                .prop_map(LinkedNode::from_node),
                1..5,
            )
        } else {
            prop::collection::vec(
                prop::strategy::Union::new(vec![
                    Number::arbitrary()
                        .prop_map(|v| Node::Value(Value::Number(v)))
                        .boxed(),
                    Boolean::arbitrary()
                        .prop_map(|v| Node::Value(Value::Boolean(v)))
                        .boxed(),
                    PrimitiveString::arbitrary()
                        .prop_map(|v| Node::Value(Value::PrimitiveString(v)))
                        .boxed(),
                    InterpolatedString::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::InterpolatedString(v)))
                        .boxed(),
                    Array::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Value(Value::Array(v)))
                        .boxed(),
                    Variable::arbitrary()
                        .prop_map(|v| Node::Expression(Expression::Variable(v)))
                        .boxed(),
                    Command::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::Command(v)))
                        .boxed(),
                    FunctionCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::FunctionCall(v)))
                        .boxed(),
                    TaskCall::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::TaskCall(v)))
                        .boxed(),
                    CompoundAssignments::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Expression(Expression::CompoundAssignments(v)))
                        .boxed(),
                    Assignation::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Assignation(v)))
                        .boxed(),
                    Break::arbitrary()
                        .prop_map(|v| Node::Statement(Statement::Break(v)))
                        .boxed(),
                    Return::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Return(v)))
                        .boxed(),
                    Each::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Each(v)))
                        .boxed(),
                    For::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::For(v)))
                        .boxed(),
                    If::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::If(v)))
                        .boxed(),
                    Join::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Join(v)))
                        .boxed(),
                    Loop::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Loop(v)))
                        .boxed(),
                    OneOf::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::OneOf(v)))
                        .boxed(),
                    Optional::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::Optional(v)))
                        .boxed(),
                    While::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Statement(Statement::While(v)))
                        .boxed(),
                    VariableDeclaration::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Declaration(Declaration::VariableDeclaration(v)))
                        .boxed(),
                    FunctionDeclaration::arbitrary_with(deep + 1)
                        .prop_map(|v| Node::Declaration(Declaration::FunctionDeclaration(v)))
                        .boxed(),
                ])
                .prop_map(LinkedNode::from_node),
                1..5,
            )
        }
        .prop_map(move |nodes| Block {
            nodes,
            open: Token::for_test(Kind::LeftBrace),
            close: Token::for_test(Kind::RightBrace),
            uuid: Uuid::new_v4(),
        })
        .boxed()
    }
}
