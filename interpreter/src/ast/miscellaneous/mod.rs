mod conflict;
mod interest;
mod link;
mod read;

mod comment;
mod include;
mod meta;
mod module;

pub use comment::*;
pub use include::*;
pub use meta::*;
pub use module::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Miscellaneous {
    /// include "path_to_scenario"
    Include(Include),
    /// mod "path_to_module"
    Module(Module),
    /// /// message
    Meta(Meta),
    /// // comment
    Comment(Comment),
}
