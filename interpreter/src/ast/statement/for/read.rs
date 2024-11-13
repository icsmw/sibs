use lexer::{Kind, KindId};

use crate::*;

impl ReadNode<For> for For {
    fn read(parser: &mut Parser) -> Result<Option<For>, E> {
        let Some(token_for) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_for.kind, Kind::For) {
            return Ok(None);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let Some(el_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedElementDeclarationInFor);
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma);
        } else {
            let _ = inner.token();
        }
        let Some(index_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?.map(Node::Expression)
        else {
            return Err(E::MissedIndexDeclarationInFor);
        };
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        };
        let Some(token_in) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_in.kind, Kind::In) {
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
            return Err(E::FailRecognizeElementsInFor(parser.to_string()));
        };
        let Some(block) = Statement::try_oneof(parser, &[StatementId::Block])?.map(Node::Statement)
        else {
            return Err(E::MissedBlock);
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
