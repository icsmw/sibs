#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for FunctionCall {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..) | Kind::Bang)
    }
}

impl ReadNode<FunctionCall> for FunctionCall {
    fn read(parser: &Parser) -> Result<Option<FunctionCall>, LinkedErr<E>> {
        let mut reference = Vec::new();
        let restore = parser.pin();
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        let negation = if matches!(token.kind, Kind::Bang) {
            Some(token)
        } else {
            restore(parser);
            None
        };
        while let Some(tk) = parser.token() {
            if let Kind::Identifier(ident) = &tk.kind {
                reference.push((ident.to_owned(), tk.clone()));
                if parser.is_next(KindId::LeftParen) {
                    break;
                }
                if let Some(tks) = parser.tokens(2) {
                    if tks
                        .into_iter()
                        .filter(|tk| tk.id() == KindId::Colon)
                        .count()
                        == 2
                    {
                        continue;
                    }
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
                    ValueId::InterpolatedString,
                    ValueId::Closure,
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
                    return Err(E::MissedComma.link_with_token(&tk.to_owned()));
                }
            } else {
                break;
            }
        }
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
        } else {
            Ok(Some(FunctionCall {
                args,
                reference,
                open: open.to_owned(),
                close: close.to_owned(),
                negation: negation.map(|tk| tk.to_owned()),
                uuid: Uuid::new_v4(),
            }))
        }
    }
}
