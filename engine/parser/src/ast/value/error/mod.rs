#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Error {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Error))
    }
}

impl ReadNode<Error> for Error {
    fn read(parser: &Parser) -> Result<Option<Error>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Error)) {
            return Ok(None);
        }
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let node = LinkedNode::try_oneof(
            &mut inner,
            &[
                NodeTarget::Value(&[
                    ValueId::Number,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                ]),
                NodeTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        .ok_or_else(|| E::MissedErrorMessage.link_with_token(&token))?;
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
        } else {
            Ok(Some(Error {
                token: token.clone(),
                node: Box::new(node),
                uuid: Uuid::new_v4(),
                open: open.clone(),
                close: close.clone(),
            }))
        }
    }
}
