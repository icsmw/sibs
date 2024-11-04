mod conflict;
mod interest;
mod read;

mod comment;
mod meta;

pub use comment::*;
pub use meta::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    Meta(Meta),
    Comment(Comment),
}
