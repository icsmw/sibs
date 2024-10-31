mod conflict;
mod interest;
mod read;

mod comment;
mod meta;

pub use comment::*;
pub use meta::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    Meta(Meta),
    Comment(Comment),
}

impl fmt::Display for MiscellaneousId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Meta => "Miscellaneous::Meta",
                Self::Comment => "Miscellaneous::Comment",
            }
        )
    }
}
