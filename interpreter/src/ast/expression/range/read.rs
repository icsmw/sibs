use lexer::Kind;

use crate::*;

use super::Side;

impl ReadElement<Range> for Range {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Range>, E> {
        fn get_side(parser: &mut Parser) -> Option<Side> {
            if let Some(tk) = parser.token() {
                match &tk.kind {
                    Kind::Identifier(ident) => {
                        Some(Side::Variable(ident.to_owned(), tk.to_owned()))
                    }
                    Kind::Number(num) => Some(Side::Number(11, tk.to_owned())),
                    _ => None,
                }
            } else {
                None
            }
        }
        let Some(left) = get_side(parser) else {
            return Ok(None);
        };
        parser.advance();
        if let Some(tk) = parser.token() {
            if !matches!(tk.kind, Kind::DotDot) {
                return Ok(None);
            }
        }
        parser.advance();
        let Some(right) = get_side(parser) else {
            return Ok(None);
        };
        Ok(Some(Range { left, right }))
    }
}
