use crate::*;
use ast::value;
use asttree::*;
use lexer::*;
use parser::*;
use proptest::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
struct TypesTest {
    pub declaration: Node,
    pub assignation: Node,
}

impl Into<Block> for TypesTest {
    fn into(self) -> Block {
        Block {
            nodes: vec![self.declaration.clone(), self.assignation.clone()],
            open: Token::for_test(Kind::LeftBrace),
            close: Token::for_test(Kind::RightBrace),
            uuid: Uuid::new_v4(),
        }
    }
}

impl Arbitrary for TypesTest {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: ()) -> Self::Strategy {
        (
            Variable::arbitrary().prop_map(|v| Node::Expression(Expression::Variable(v))),
            prop::strategy::Union::new(vec![
                Boolean::arbitrary()
                    .prop_map(Value::Boolean)
                    .prop_map(Node::Value)
                    .boxed(),
                Number::arbitrary()
                    .prop_map(Value::Number)
                    .prop_map(Node::Value)
                    .boxed(),
                PrimitiveString::arbitrary()
                    .prop_map(Value::PrimitiveString)
                    .prop_map(Node::Value)
                    .boxed(),
            ]),
        )
            .prop_map(|(variable, value)| {
                let ty = match &value {
                    Node::Value(Value::Boolean(_)) => VariableType {
                        r#type: VariableTypeDef::Primitive(Token::for_test(Kind::Keyword(
                            Keyword::Bool,
                        ))),
                        uuid: Uuid::new_v4(),
                    },
                    Node::Value(Value::Number(_)) => VariableType {
                        r#type: VariableTypeDef::Primitive(Token::for_test(Kind::Keyword(
                            Keyword::Num,
                        ))),
                        uuid: Uuid::new_v4(),
                    },
                    Node::Value(Value::PrimitiveString(_)) => VariableType {
                        r#type: VariableTypeDef::Primitive(Token::for_test(Kind::Keyword(
                            Keyword::Str,
                        ))),
                        uuid: Uuid::new_v4(),
                    },
                    _ => VariableType {
                        r#type: VariableTypeDef::Primitive(Token::for_test(Kind::Keyword(
                            Keyword::Bool,
                        ))),
                        uuid: Uuid::new_v4(),
                    },
                };
                let value = Node::Statement(Statement::AssignedValue(AssignedValue {
                    token: Token::for_test(Kind::Equals),
                    node: Box::new(value.clone()),
                    uuid: Uuid::new_v4(),
                }));
                TypesTest {
                    declaration: Node::Declaration(Declaration::VariableDeclaration(
                        VariableDeclaration {
                            token: Token::for_test(Kind::Keyword(Keyword::Let)),
                            variable: Box::new(variable.clone()),
                            r#type: Some(Box::new(Node::Declaration(
                                Declaration::VariableTypeDeclaration(VariableTypeDeclaration {
                                    types: vec![Node::Declaration(Declaration::VariableType(ty))],
                                    token: Token::for_test(Kind::Colon),
                                    uuid: Uuid::new_v4(),
                                }),
                            ))),
                            assignation: Some(Box::new(value.clone())),
                            uuid: Uuid::new_v4(),
                        },
                    )),
                    assignation: Node::Statement(Statement::Assignation(Assignation {
                        left: Box::new(variable),
                        right: Box::new(value),
                        uuid: Uuid::new_v4(),
                    })),
                }
            })
            .boxed()
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(500)
    })]

    #[test]
    fn test(cases in proptest::collection::vec(TypesTest::arbitrary(), 10)) {
        for case in cases.into_iter() {
            test_node_success( Node::Statement(Statement::Block(case.into())));
        }
    }

}
