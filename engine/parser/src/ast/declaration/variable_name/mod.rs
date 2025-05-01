#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for VariableName {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Identifier(..))
    }
}

impl ReadNode<VariableName> for VariableName {
    fn read(parser: &Parser) -> Result<Option<VariableName>, LinkedErr<E>> {
        if let Some(tk) = parser.token() {
            let Kind::Identifier(ident) = &tk.kind else {
                return Ok(None);
            };
            return Ok(Some(VariableName {
                ident: ident.clone(),
                token: tk.to_owned(),
                uuid: Uuid::new_v4(),
            }));
        }
        Ok(None)
    }
}
