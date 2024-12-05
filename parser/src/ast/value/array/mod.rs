#[cfg(test)]
mod proptests;

use crate::*;

impl ReadNode<Array> for Array {
    fn read(parser: &mut Parser) -> Result<Option<Array>, LinkedErr<E>> {
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftBracket, KindId::RightBracket)?
        else {
            return Ok(None);
        };
        let mut els = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[
                NodeReadTarget::Value(&[
                    ValueId::Number,
                    ValueId::Boolean,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                ]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExpSeq,
                    ExpressionId::ComparisonSeq,
                    ExpressionId::FunctionCall,
                    ExpressionId::Command,
                ]),
            ],
        )? {
            els.push(node);
            if let Some(tk) = inner.token() {
                if tk.id() != KindId::Comma {
                    return Err(E::MissedComma.link_by_current(&inner));
                }
            } else {
                break;
            }
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
        } else {
            Ok(Some(Array {
                els,
                open,
                close,
                uuid: Uuid::new_v4(),
            }))
        }
    }
}
