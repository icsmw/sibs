use lexer::Kind;

use crate::*;

impl ReadNode<BinaryOp> for BinaryOp {
    fn read(parser: &mut Parser) -> Result<Option<BinaryOp>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        let operator = match tk.kind {
            Kind::Plus => BinaryOperator::Plus,
            Kind::Minus => BinaryOperator::Minus,
            Kind::Star => BinaryOperator::Star,
            Kind::Slash => BinaryOperator::Slash,
            _ => return Ok(None),
        };
        Ok(Some(BinaryOp {
            token: tk.clone(),
            operator,
        }))
    }
}
