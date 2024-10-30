mod conflict;
mod interest;
mod read;

mod command;

pub use command::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    // Block(Block),
    Command(Command),
    // Meta(Meta),
    // Comment(Comment),
}

impl fmt::Display for MiscellaneousId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Command => "Miscellaneous::Command",
            }
        )
    }
}
