use lexer::Kind;

use crate::*;

impl ReadNode<Optional> for Optional {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Optional>, E> {
        let Some(comparison) =
            Expression::try_oneof(parser, &[ExpressionId::ComparisonSeq], nodes)?
                .map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::DoubleArrow) {
            return Ok(None);
        }
        let Some(action) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Statement(&[
                    StatementId::Break,
                    StatementId::Return,
                    StatementId::Block,
                    StatementId::Loop,
                    StatementId::For,
                    StatementId::While,
                    StatementId::Assignation,
                    StatementId::Each,
                    StatementId::Join,
                    StatementId::OneOf,
                ]),
                NodeReadTarget::Expression(&[ExpressionId::Command, ExpressionId::FunctionCall]),
            ],
        )?
        else {
            return Err(E::MissedActionInOptional);
        };
        Ok(Some(Optional {
            token,
            action: Box::new(action),
            comparison: Box::new(comparison),
        }))
    }
}
