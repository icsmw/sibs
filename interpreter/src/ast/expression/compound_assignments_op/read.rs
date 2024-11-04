use lexer::Kind;

use crate::*;

impl ReadElement<CompoundAssignmentsOp> for CompoundAssignmentsOp {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<CompoundAssignmentsOp>, E> {
        let Some(tk) = parser.token() else {
            return Ok(None);
        };
        let operator = match tk.kind {
            Kind::PlusEqual => CompoundAssignmentsOperator::PlusEqual,
            Kind::MinusEqual => CompoundAssignmentsOperator::MinusEqual,
            Kind::StarEqual => CompoundAssignmentsOperator::StarEqual,
            Kind::SlashEqual => CompoundAssignmentsOperator::SlashEqual,
            _ => return Ok(None),
        };
        Ok(Some(CompoundAssignmentsOp {
            token: tk.clone(),
            operator,
        }))
    }
}
