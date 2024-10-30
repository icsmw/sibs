mod assignation;
mod r#break;
mod each;
mod r#for;
mod r#loop;
mod r#return;
mod r#while;

mod conflict;
mod interest;
mod read;

pub use assignation::*;
pub use each::*;
pub use r#break::*;
pub use r#for::*;
pub use r#loop::*;
pub use r#return::*;
pub use r#while::*;

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
    For(For),
    While(While),
    Loop(Loop),
    Each(Each),
    // Conclusion(Conclusion),
    Assignation(Assignation),
}

impl fmt::Display for StatementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Break => "Statement::Break",
                Self::Return => "Statement::Return",
                Self::For => "Statement::For",
                Self::While => "Statement::While",
                Self::Loop => "Statement::Loop",
                Self::Each => "Statement::Each",
                Self::Assignation => "Statement::Assignation",
            }
        )
    }
}
