use lexer::Kind;

use crate::*;

impl ReadNode<CompoundAssignmentsOp> for CompoundAssignmentsOp {
    fn read(parser: &mut Parser) -> Result<Option<CompoundAssignmentsOp>, LinkedErr<E>> {
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
