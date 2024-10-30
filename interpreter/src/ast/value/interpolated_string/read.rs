use lexer::Kind;

use crate::*;

impl ReadElement<InterpolatedString> for InterpolatedString {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<InterpolatedString>, E> {
        if let Some(tk) = parser.token() {
            let Kind::InterpolatedString(_) = &tk.kind else {
                return Ok(None);
            };
        }
        Ok(None)
    }
}
