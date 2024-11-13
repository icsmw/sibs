use lexer::{Keyword, Kind, KindId};

use crate::*;

impl ReadNode<OneOf> for OneOf {
    fn read(parser: &mut Parser) -> Result<Option<OneOf>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::OneOf)) {
            return Ok(None);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let mut commands = Vec::new();
        while let Some(node) =
            Expression::try_read(&mut inner, ExpressionId::Command)?.map(Node::Expression)
        {
            commands.push(node);
            let Some(tk) = inner.token() else {
                continue;
            };
            if !matches!(tk.kind, Kind::Comma) {
                return Err(E::MissedComma);
            }
        }
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        };
        Ok(Some(OneOf { commands, token }))
    }
}
