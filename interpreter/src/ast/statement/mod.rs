mod conflict;
mod interest;
mod link;
mod read;

mod assignation;
mod assigned_value;
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
pub use assigned_value::*;
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

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Statement {
    /// { ... }
    Block(Block),
    /// break;
    Break(Break),
    /// return;
    Return(Return),
    /// a > 5 => func();
    /// a < 4 || b > 5 => a = 5;
    /// (a < 100 || b > 100) && v != 0 => { ... };
    /// a > 5 => break;
    /// a > 5 => return;
    /// a > 5 => loop ...
    /// a > 5 => while ...
    /// a > 5 => for ...
    /// a > 5 => each ...
    /// a > 5 => oneof ...
    /// a > 5 => join ...
    Optional(Optional),
    /// if a > 4 { ... }
    /// if a > 5 { ... } if b > 5 { ... }
    /// if a > 5 { ... } if b > 5 { ... } else { ... }
    If(If),
    /// for (el, n) in 0..1 { ... };
    /// for (el, n) in from..to { ... };
    /// for (el, n) in elements { ... };
    /// for (el, n) in [1, 2, 3] { ... };
    /// for (el, n) in ["one", "two", "three"] { ... };
    For(For),
    /// while s < 100 { ... };
    /// while a < 100 || b > 100 { ... };
    /// while (a < 100 || b > 100) && v != 0 { ... };
    While(While),
    /// loop { ... }
    Loop(Loop),
    /// each(el, n, elements) { el; };
    /// each(el, n, [1, 2, 3]) { el; };
    Each(Each),
    /// a = 1, a = func(), etc.
    Assignation(Assignation),
    /// any value to assignate to variable
    AssignedValue(AssignedValue),
    OneOf(OneOf),
    Join(Join),
}
