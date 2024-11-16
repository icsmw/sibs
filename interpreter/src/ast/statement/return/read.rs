use lexer::{Keyword, Kind, KindId};

use crate::*;

impl ReadNode<Return> for Return {
    fn read(parser: &mut Parser) -> Result<Option<Return>, E> {
        let Some(tk) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(tk.kind, Kind::Keyword(Keyword::Return)) {
            return Ok(None);
        }
        let node = if parser.is_next(KindId::Semicolon) || parser.is_done() {
            None
        } else {
            Some(
                Node::try_oneof(
                    parser,
                    &[
                        NodeReadTarget::Value(&[
                            ValueId::Number,
                            ValueId::Boolean,
                            ValueId::PrimitiveString,
                            ValueId::InterpolatedString,
                            ValueId::Error,
                        ]),
                        NodeReadTarget::Expression(&[
                            ExpressionId::Variable,
                            ExpressionId::BinaryExpSeq,
                            ExpressionId::ComparisonSeq,
                            ExpressionId::FunctionCall,
                        ]),
                    ],
                )?
                .map(Box::new)
                .ok_or(E::InvalidReturnValue)?,
            )
        };
        Ok(Some(Return {
            token: tk.to_owned(),
            node,
        }))
    }
}
