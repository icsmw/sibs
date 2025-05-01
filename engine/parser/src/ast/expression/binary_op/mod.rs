#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for BinaryOp {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Plus | Kind::Minus | Kind::Star | Kind::Slash
        )
    }
}

impl ReadNode<BinaryOp> for BinaryOp {
    fn read(parser: &Parser) -> Result<Option<BinaryOp>, LinkedErr<E>> {
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
            uuid: Uuid::new_v4(),
        }))
    }
}
