#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for CompoundAssignmentsOp {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::PlusEqual | Kind::MinusEqual | Kind::StarEqual | Kind::SlashEqual
        )
    }
}

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
            uuid: Uuid::new_v4(),
        }))
    }
}
