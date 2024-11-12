use lexer::Kind;

use crate::*;

impl ReadNode<VariableDeclaration> for VariableDeclaration {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<VariableDeclaration>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::Let) {
            return Ok(None);
        }
        let Some(variable) = parser.token().cloned() else {
            return Err(E::MissedVariableDefinition);
        };
        let Some(tk) = parser.token() else {
            return Ok(Some(VariableDeclaration {
                token,
                variable,
                r#type: None,
                value: None,
            }));
        };
        match tk.kind {
            Kind::Colon => {
                if let Some(node) =
                    Declaration::try_read(parser, DeclarationId::VariableType, nodes)?
                        .map(Node::Declaration)
                {
                    Ok(Some(VariableDeclaration {
                        token,
                        variable,
                        r#type: Some(Box::new(node)),
                        value: None,
                    }))
                } else {
                    Err(E::MissedVariableTypeDefinition)
                }
            }
            Kind::Equals => {
                if let Some(node) = Statement::try_read(parser, StatementId::AssignedValue, nodes)?
                    .map(Node::Statement)
                {
                    Ok(Some(VariableDeclaration {
                        token,
                        variable,
                        r#type: None,
                        value: Some(Box::new(node)),
                    }))
                } else {
                    Err(E::InvalidAssignation(parser.to_string()))
                }
            }
            _ => Ok(Some(VariableDeclaration {
                token,
                variable,
                r#type: None,
                value: None,
            })),
        }
    }
}
