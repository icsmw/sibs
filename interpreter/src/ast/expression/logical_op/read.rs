use lexer::Kind;

use crate::*;

impl ReadElement<LogicalOp> for LogicalOp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<LogicalOp>, E> {
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
