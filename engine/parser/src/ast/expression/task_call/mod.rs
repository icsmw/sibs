#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for TaskCall {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Colon)
    }
}

impl ReadNode<TaskCall> for TaskCall {
    fn read(parser: &Parser) -> Result<Option<TaskCall>, LinkedErr<E>> {
        let mut reference = Vec::new();
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        if !matches!(tk.kind, Kind::Colon) {
            return Ok(None);
        }
        while let Some(tk) = parser.token() {
            if let Kind::Identifier(ident) = &tk.kind {
                reference.push((ident.to_owned(), tk.clone()));
                if parser.is_next(KindId::LeftParen) {
                    break;
                }
                if parser.is_next(KindId::Colon) {
                    let _ = parser.token();
                    continue;
                }
            }
            return Ok(None);
        }
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let mut args = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[
                NodeTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::Array,
                ]),
                NodeTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::ComparisonSeq,
                    ExpressionId::FunctionCall,
                ]),
            ],
        )? {
            args.push(node);
            if let Some(tk) = inner.token() {
                if tk.id() != KindId::Comma {
                    return Err(E::MissedComma.link_with_token(&tk));
                }
            } else {
                break;
            }
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
        } else {
            Ok(Some(TaskCall {
                args,
                reference,
                open: open.clone(),
                close: close.clone(),
                uuid: Uuid::new_v4(),
            }))
        }
    }
}
