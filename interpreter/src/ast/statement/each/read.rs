use lexer::{Kind, KindId};

use crate::*;

impl ReadNode<Each> for Each {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Each>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Each) {
            return Ok(None);
        }
        let Some(mut inner) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let Some(el_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable], &Nodes::empty())?
                .map(Node::Expression)
        else {
            return Err(E::MissedElementDeclarationInEach);
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma);
        } else {
            let _ = inner.token();
        }
        let Some(index_ref) =
            Expression::try_oneof(&mut inner, &[ExpressionId::Variable], &Nodes::empty())?
                .map(Node::Expression)
        else {
            return Err(E::MissedIndexDeclarationInEach);
        };
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma);
        } else {
            let _ = inner.token();
        }
        let Some(elements) = Node::try_oneof(
            &mut inner,
            &Nodes::empty(),
            &[
                NodeReadTarget::Value(&[ValueId::Array]),
                NodeReadTarget::Expression(&[ExpressionId::Variable, ExpressionId::FunctionCall]),
            ],
        )?
        else {
            return Err(E::FailRecognizeElementsInEach(inner.to_string()));
        };
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()));
        };
        let Some(block) =
            Statement::try_oneof(parser, &[StatementId::Block], nodes)?.map(Node::Statement)
        else {
            return Err(E::MissedBlockInEach);
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
