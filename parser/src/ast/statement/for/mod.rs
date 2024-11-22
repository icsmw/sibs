#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind, KindId};

impl ReadNode<For> for For {
    fn read(parser: &mut Parser) -> Result<Option<For>, LinkedErr<E>> {
        let Some(token_for) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_for.kind, Kind::Keyword(Keyword::For)) {
            return Ok(None);
        }
        let Some((mut inner, ..)) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let el_ref = Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?
            .map(Node::Expression)
            .ok_or_else(|| E::MissedElementDeclarationInFor.link_with_token(&token_for))?;
        if !inner.is_next(KindId::Comma) {
            return Err(E::MissedComma.link_by_current(&inner));
        } else {
            let _ = inner.token();
        }
        let index_ref = Expression::try_oneof(&mut inner, &[ExpressionId::Variable])?
            .map(Node::Expression)
            .ok_or_else(|| E::MissedIndexDeclarationInFor.link_by_current(&inner))?;
        if !inner.is_done() {
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        };
        let Some(token_in) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token_in.kind, Kind::Keyword(Keyword::In)) {
            return Ok(None);
        }
        let elements = Node::try_oneof(
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
        .ok_or_else(|| {
            E::FailRecognizeElementsInFor(parser.to_string()).link_with_token(&token_in)
        })?;
        let block = Statement::try_oneof(parser, &[StatementId::Block])?
            .map(Node::Statement)
            .ok_or_else(|| E::MissedBlock.link_with_token(&token_for))?;
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
