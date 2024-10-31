mod conflict;
mod interest;
mod read;

mod assignation;
mod block;
mod r#break;
mod each;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

pub use assignation::*;
pub use block::*;
pub use each::*;
pub use join::*;
pub use oneof::*;
pub use optional::*;
pub use r#break::*;
pub use r#for::*;
pub use r#if::*;
pub use r#loop::*;
pub use r#return::*;
pub use r#while::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Statement {
    Block(Block),
    Break(Break),
    Return(Return),
    Optional(Optional),
    If(If),
    For(For),
    While(While),
    Loop(Loop),
    Each(Each),
    Assignation(Assignation),
    OneOf(OneOf),
    Join(Join),
}

impl fmt::Display for StatementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Block => "Statement::Block",
                Self::Break => "Statement::Break",
                Self::Return => "Statement::Return",
                Self::For => "Statement::For",
                Self::While => "Statement::While",
                Self::Loop => "Statement::Loop",
                Self::Each => "Statement::Each",
                Self::Assignation => "Statement::Assignation",
                Self::Optional => "Statement::Optional",
                Self::OneOf => "Statement::OneOf",
                Self::Join => "Statement::Join",
                Self::If => "Statement::If",
            }
        )
    }
}
