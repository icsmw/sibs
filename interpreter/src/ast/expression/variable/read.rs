use lexer::Kind;

use crate::*;

impl ReadNode<Variable> for Variable {
    fn read(parser: &mut Parser) -> Result<Option<Variable>, E> {
        if let Some(tk) = parser.token() {
            let Kind::Identifier(ident) = &tk.kind else {
                return Ok(None);
            };
            return Ok(Some(Variable {
                ident: ident.clone(),
                token: tk.to_owned(),
            }));
        }
        Ok(None)
    }
}
