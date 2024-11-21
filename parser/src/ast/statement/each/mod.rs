mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<Each> for Each {
    fn read(parser: &mut Parser) -> Result<Option<Each>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Each)) {
            return Ok(None);
        }
        let Some((mut inner, ..)) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let Some(el_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedElementDeclarationInEach.link_with_token(&token));
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let Some(index_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedIndexDeclarationInEach.link_with_token(&token));
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let Some(elements) = Node::try_oneof(
            &mut inner,
            &[
                NodeReadTarget::Value(&[ValueId::Array]),
                NodeReadTarget::Expression(&[ExpressionId::Variable, ExpressionId::FunctionCall]),
            ],
        )?
        else {
            return Err(E::FailRecognizeElementsInEach(inner.to_string()).link_until_end(&inner));
        };
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        };
        let Some(block) = Statement::try_oneof(parser, &[StatementId::Block])?.map(Node::Statement)
        else {
            return Err(E::MissedBlock.link_with_token(&token));
        };
        Ok(Some(Each {
            token,
            element: Box::new(el_ref),
            index: Box::new(index_ref),
            elements: Box::new(elements),
            block: Box::new(block),
        }))
    }
}
