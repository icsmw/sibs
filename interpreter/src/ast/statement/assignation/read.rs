use lexer::Kind;

use crate::*;

impl ReadNode<Assignation> for Assignation {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Assignation>, E> {
        let Some(left) =
            Expression::try_oneof(parser, &[ExpressionId::Variable], nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Equals) {
            return Ok(None);
        };
        let Some(right) = Node::try_oneof(
            parser,
            nodes,
            &[
                NodeReadTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                    ValueId::Array,
                ]),
                NodeReadTarget::Statement(&[StatementId::If]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::ComparisonSeq,
                    ExpressionId::FunctionCall,
                    ExpressionId::Command,
                    ExpressionId::TaskCall,
                ]),
            ],
        )?
        else {
            return Err(E::InvalidAssignation(parser.to_string()));
        };
        Ok(Some(Assignation {
            left: Box::new(left),
            token,
            right: Box::new(right),
        }))
    }
}
