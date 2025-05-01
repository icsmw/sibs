#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Block {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::LeftBrace)
    }
}

impl ReadNode<Block> for Block {
    fn read(parser: &Parser) -> Result<Option<Block>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftBrace, KindId::RightBrace)?
        else {
            return Ok(None);
        };
        let mut nodes = Vec::new();
        loop {
            'semicolons: loop {
                if inner.is_next(KindId::Semicolon) {
                    let _ = inner.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(node) = LinkedNode::try_oneof(
                &mut inner,
                &[
                    NodeTarget::Declaration(&[
                        DeclarationId::VariableDeclaration,
                        DeclarationId::FunctionDeclaration,
                    ]),
                    NodeTarget::Statement(&[
                        StatementId::Assignation,
                        StatementId::Break,
                        StatementId::Return,
                        StatementId::For,
                        StatementId::If,
                        StatementId::Join,
                        StatementId::Loop,
                        StatementId::OneOf,
                        StatementId::Optional,
                        StatementId::While,
                    ]),
                    NodeTarget::Expression(&[
                        ExpressionId::Command,
                        ExpressionId::FunctionCall,
                        ExpressionId::TaskCall,
                        ExpressionId::CompoundAssignments,
                        ExpressionId::Variable,
                        ExpressionId::BinaryExpSeq,
                        ExpressionId::ComparisonSeq,
                        ExpressionId::Comparison,
                    ]),
                    NodeTarget::Value(&[
                        ValueId::Boolean,
                        ValueId::Number,
                        ValueId::InterpolatedString,
                        ValueId::PrimitiveString,
                        ValueId::Array,
                        ValueId::Error,
                    ]),
                ],
            )?
            else {
                break;
            };
            nodes.push(node);
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
        } else {
            Ok(Some(Block {
                nodes,
                open,
                close,
                uuid: Uuid::new_v4(),
            }))
        }
    }
}
