use lexer::{Kind, KindId};

use crate::*;

impl ReadNode<Error> for Error {
    fn read(parser: &mut Parser) -> Result<Option<Error>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::Identifier(ident) = &token.kind else {
            return Ok(None);
        };
        if ident != "Error" {
            return Ok(None);
        }
        let Some((mut inner, ..)) =  parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let Some(node) = Node::try_oneof(
            &mut inner,
            &[
                NodeReadTarget::Value(&[
                    ValueId::Number,
                    ValueId::PrimitiveString,
                    ValueId::InterpolatedString,
                ]),
                NodeReadTarget::Expression(&[ExpressionId::Variable]),
            ],
        )?
        else {
            return Err(E::MissedErrorMessage.link_with_token(&token));
        };
        if !inner.is_done() {
            Err(E::UnrecognizedCode(inner.to_string()).link_from_current(&inner))
        } else {
            Ok(Some(Error {
                token,
                node: Box::new(node),
            }))
        }
    }
}
