use lexer::Kind;

use super::Side;
use crate::*;

impl ReadElement<Comparing> for Comparing {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<Comparing>, E> {
        fn get_side(parser: &mut Parser) -> Result<Option<Side>, E> {
            let Some(tk) = parser.token() else {
                return Ok(None);
            };
            match &tk.kind {
                Kind::Identifier(ident) => {
                    Ok(Some(Side::Variable(ident.to_owned(), tk.to_owned())))
                }
                Kind::Number(num) => {
                    if num.is_finite() {
                        Ok(Some(Side::Number(num.round() as i64, tk.to_owned())))
                    } else {
                        Err(E::InfiniteNumber)
                    }
                }

                _ => Ok(None),
            }
        }
        let Some(left) = get_side(parser)? else {
            return Ok(None);
        };
        parser.advance();
        if let Some(tk) = parser.token() {
            if !matches!(tk.kind, Kind::DotDot) {
                return Ok(None);
            }
        }
        Ok(None)
    }
}
