use lexer::KindId;

use crate::*;

impl ReadNode<Array> for Array {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Array>, E> {
        let Some(mut inner) = parser.between(KindId::LeftBracket, KindId::RightBracket)? else {
            return Ok(None);
        };
        let mut els = Vec::new();
        while let Some(node) = Node::try_oneof(
            &mut inner,
            nodes,
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
                    return Err(E::MissedComma);
                }
            } else {
                break;
            }
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()))
        } else {
            Ok(Some(Array { els }))
        }
    }
}
