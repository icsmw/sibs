mod r#break;
mod conflict;
mod interest;
mod read;
mod r#return;

pub use r#break::*;
pub use r#return::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Statement {
    Break(Break),
    Return(Return),
    // If(If),
    // IfCondition(IfCondition),
    // IfSubsequence(IfSubsequence),
    // IfThread(IfThread),
    // For(For),
    // While(While),
    // Loop(Loop),
    // Each(Each),
    // Conclusion(Conclusion),
    // VariableAssignation(VariableAssignation),
}

impl fmt::Display for StatementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Break => "Statement::Break",
                Self::Return => "Statement::Return",
            }
        )
    }
}
