#[cfg(test)]
mod proptests;

use crate::*;

impl ReadNode<OneOf> for OneOf {
    fn read(parser: &mut Parser) -> Result<Option<OneOf>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::OneOf)) {
            return Ok(None);
        }
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftParen, KindId::RightParen)?
        else {
            return Ok(None);
        };
        let mut commands = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            &mut inner,
            &[NodeReadTarget::Expression(&[ExpressionId::Command])],
        )? {
            commands.push(node);
            let Some(tk) = inner.token() else {
                continue;
            };
            if !matches!(tk.kind, Kind::Comma) {
                return Err(E::MissedComma.link_with_token(tk));
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        };
        Ok(Some(OneOf {
            commands,
            token,
            open,
            close,
            uuid: Uuid::new_v4(),
        }))
    }
}
