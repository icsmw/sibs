#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Optional {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::LeftParen
                | Kind::Identifier(..)
                | Kind::Number(..)
                | Kind::String(..)
                | Kind::SingleQuote
                | Kind::Keyword(Keyword::True)
                | Kind::Keyword(Keyword::False)
                | Kind::Bang
        )
    }
}

impl ReadNode<Optional> for Optional {
    fn read(parser: &Parser) -> Result<Option<Optional>, LinkedErr<E>> {
        let Some(comparison) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[ExpressionId::ComparisonSeq])],
        )?
        else {
            return Ok(None);
        };
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::DoubleArrow) {
            return Ok(None);
        }
        let action = LinkedNode::try_oneof(
            parser,
            &[
                NodeTarget::Statement(&[
                    StatementId::Break,
                    StatementId::Return,
                    StatementId::Block,
                    StatementId::Loop,
                    StatementId::For,
                    StatementId::While,
                    StatementId::Assignation,
                    StatementId::Join,
                    StatementId::OneOf,
                ]),
                NodeTarget::Expression(&[ExpressionId::Command, ExpressionId::FunctionCall]),
            ],
        )?
        .ok_or_else(|| E::MissedActionInOptional.link_with_token(&token))?;
        Ok(Some(Optional {
            token: token.clone(),
            action: Box::new(action),
            comparison: Box::new(comparison),
            uuid: Uuid::new_v4(),
        }))
    }
}
