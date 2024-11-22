#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Return> for Return {
    fn read(parser: &mut Parser) -> Result<Option<Return>, LinkedErr<E>> {
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
                .ok_or_else(|| E::InvalidReturnValue.link_with_token(&tk))?,
            )
        };
        Ok(Some(Return {
            token: tk.to_owned(),
            node,
        }))
    }
}
