#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Variable {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..) | Kind::Bang)
    }
}

impl ReadNode<Variable> for Variable {
    fn read(parser: &Parser) -> Result<Option<Variable>, LinkedErr<E>> {
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        let (token, negation) = if matches!(token.kind, Kind::Bang) {
            let Some(next) = parser.token() else {
                return Ok(None);
            };
            (next, Some(token))
        } else {
            (token, None)
        };
        let Kind::Identifier(ident) = &token.kind else {
            return Ok(None);
        };
        Ok(Some(Variable {
            ident: ident.clone(),
            negation: negation.map(|tk| tk.clone()),
            token: token.clone(),
            uuid: Uuid::new_v4(),
        }))
    }
}
