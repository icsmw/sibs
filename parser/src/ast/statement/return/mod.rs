#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Return {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Return))
    }
}

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
                LinkedNode::try_oneof(
                    parser,
                    &[
                        NodeTarget::Value(&[
                            ValueId::Number,
                            ValueId::Boolean,
                            ValueId::PrimitiveString,
                            ValueId::InterpolatedString,
                            ValueId::Error,
                        ]),
                        NodeTarget::Expression(&[
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
            targets: Vec::new(),
            uuid: Uuid::new_v4(),
        }))
    }
}
