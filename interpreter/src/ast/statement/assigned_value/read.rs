use lexer::Kind;

use crate::*;

impl ReadNode<AssignedValue> for AssignedValue {
    fn read(parser: &mut Parser) -> Result<Option<AssignedValue>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Equals) {
            return Ok(None);
        };
        let Some(node) = Node::try_oneof(
            parser,
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
        Ok(Some(AssignedValue {
            token,
            node: Box::new(node),
        }))
    }
}
