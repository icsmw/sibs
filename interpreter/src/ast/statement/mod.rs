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

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
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
