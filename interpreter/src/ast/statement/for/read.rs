use lexer::{Keyword, Kind, KindId};

use crate::*;

impl ReadNode<For> for For {
    fn read(parser: &mut Parser) -> Result<Option<For>, LinkedErr<E>> {
        let Some(token_for) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_for.kind, Kind::Keyword(Keyword::For)) {
            return Ok(None);
        }
        let Some((mut inner, ..)) =  parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let Some(el_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedElementDeclarationInFor.link_with_token(&token_for));
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let Some(index_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedIndexDeclarationInFor.link_by_current(&inner));
        };
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_from_current(&inner));
        };
        let Some(token_in) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_in.kind, Kind::Keyword(Keyword::In)) {
            return Ok(None);
        }
        let Some(elements) = Node::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Array]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::FunctionCall,
                    ExpressionId::Range,
                ]),
            ],
        )?
        else {
            return Err(
                E::FailRecognizeElementsInFor(parser.to_string()).link_with_token(&token_in)
            );
        };
        let Some(block) = Statement::try_oneof(parser, &[StatementId::Block])?.map(Node::Statement)
        else {
            return Err(E::MissedBlock.link_with_token(&token_for));
        };
        Ok(Some(For {
            token_for,
            token_in,
            element: Box::new(el_ref),
            index: Box::new(index_ref),
            elements: Box::new(elements),
            block: Box::new(block),
        }))
    }
}
