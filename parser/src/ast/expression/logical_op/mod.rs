#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::Kind;

impl ReadNode<LogicalOp> for LogicalOp {
    fn read(parser: &mut Parser) -> Result<Option<LogicalOp>, LinkedErr<E>> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        let operator = match tk.kind {
            Kind::And => LogicalOperator::And,
            Kind::Or => LogicalOperator::Or,
            _ => return Ok(None),
        };
        Ok(Some(LogicalOp {
            token: tk.clone(),
            operator,
        }))
    }
}
