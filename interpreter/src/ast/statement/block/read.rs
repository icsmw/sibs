use lexer::{Kind, KindId};

use crate::*;

impl ReadNode<Block> for Block {
    fn read(parser: &mut Parser) -> Result<Option<Block>, E> {
        let Some(mut inner) = parser.between(KindId::LeftBrace, KindId::RightBrace)? else {
            return Ok(None);
        };
        let mut nodes = Vec::new();
        while let Some(node) = Node::try_oneof(
            &mut inner,
            
            &[
                NodeReadTarget::Statement(&[
                    StatementId::Assignation,
                    StatementId::Break,
                    StatementId::Return,
                    StatementId::Each,
                    StatementId::For,
                    StatementId::If,
                    StatementId::Join,
                    StatementId::Loop,
                    StatementId::OneOf,
                    StatementId::Optional,
                    StatementId::While,
                ]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Command,
                    ExpressionId::FunctionCall,
                    ExpressionId::TaskCall,
                    ExpressionId::CompoundAssignments,
                    ExpressionId::Variable,
                ]),
                NodeReadTarget::Value(&[
                    ValueId::Boolean,
                    ValueId::Number,
                    ValueId::InterpolatedString,
                    ValueId::PrimitiveString,
                    ValueId::Array,
                ]),
            ],
        )? {
            nodes.push(node);
            if let Some(tk) = inner.token() {
                if !matches!(tk.kind, Kind::Semicolon) {
                    return Err(E::MissedSemicolon);
                }
            } else {
                break;
            }
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()))
        } else {
            Ok(Some(Block { nodes }))
        }
    }
}
