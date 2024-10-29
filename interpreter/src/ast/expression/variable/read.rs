use lexer::Kind;

use crate::*;

impl ReadElement<Variable> for Variable {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Variable>, E> {
        if let Some(tk) = parser.token() {
            let Kind::Identifier(ident) = &tk.kind else {
                return Ok(None);
            };
            let node = Variable {
                ident: ident.clone(),
            };
            parser.advance();
            return Ok(Some(node));
        }
        Ok(None)
    }
}
