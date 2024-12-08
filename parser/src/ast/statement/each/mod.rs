#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Each {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Each))
    }
}

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
        let el_ref = LinkedNode::try_oneof(
            &mut inner,
            &[NodeReadTarget::Expression(&[ExpressionId::Variable])],
        )?
        .ok_or_else(|| E::MissedElementDeclarationInEach.link_with_token(&token))?;
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let index_ref = LinkedNode::try_oneof(
            &mut inner,
            &[NodeReadTarget::Expression(&[ExpressionId::Variable])],
        )?
        .ok_or_else(|| E::MissedIndexDeclarationInEach.link_with_token(&token))?;
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let elements = LinkedNode::try_oneof(
            &mut inner,
            &[
                NodeReadTarget::Value(&[ValueId::Array]),
                NodeReadTarget::Expression(&[ExpressionId::Variable, ExpressionId::FunctionCall]),
            ],
        )?
        .ok_or_else(|| E::FailRecognizeElementsInEach(inner.to_string()).link_until_end(&inner))?;
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        };
        let block =
            LinkedNode::try_oneof(parser, &[NodeReadTarget::Statement(&[StatementId::Block])])?
                .ok_or_else(|| E::MissedBlock.link_with_token(&token))?;
        Ok(Some(Each {
            token,
            element: Box::new(el_ref),
            index: Box::new(index_ref),
            elements: Box::new(elements),
            block: Box::new(block),
            uuid: Uuid::new_v4(),
        }))
    }
}
