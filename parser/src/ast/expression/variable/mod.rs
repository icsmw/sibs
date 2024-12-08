#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Variable {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..))
    }
}

impl ReadNode<Variable> for Variable {
    fn read(parser: &mut Parser) -> Result<Option<Variable>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let Kind::Identifier(ident) = &tk.kind else {
                return Ok(None);
            };
            return Ok(Some(Variable {
                ident: ident.clone(),
                token: tk.to_owned(),
                uuid: Uuid::new_v4(),
            }));
        }
        Ok(None)
    }
}
