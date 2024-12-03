#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::Kind;

impl ReadNode<VariableName> for VariableName {
    fn read(parser: &mut Parser) -> Result<Option<VariableName>, LinkedErr<E>> {
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
