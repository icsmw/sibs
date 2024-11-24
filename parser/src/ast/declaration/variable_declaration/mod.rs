#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Keyword(Keyword::Let)) {
            return Ok(None);
        }
        let variable = Expression::try_read(parser, ExpressionId::Variable)?
            .map(Node::Expression)
            .ok_or_else(|| E::MissedVariableDefinition.link_with_token(&token))?;
        let ty = Node::try_oneof(
            parser,
            &[NodeReadTarget::Declaration(&[
                DeclarationId::VariableTypeDeclaration,
            ])],
        )?
        .map(Box::new);
        let assignation = Node::try_oneof(
            parser,
            &[NodeReadTarget::Statement(&[StatementId::AssignedValue])],
        )?
        .map(Box::new);
        Ok(Some(VariableDeclaration {
            token,
            variable: Box::new(variable),
            r#type: ty,
            assignation,
            uuid: Uuid::new_v4(),
        }))
    }
}
