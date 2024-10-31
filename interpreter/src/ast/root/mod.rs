mod conflict;
mod interest;
mod read;

mod component;
mod task;

pub use component::*;
pub use task::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Root {
    Component(Component),
    Task(Task),
}

impl fmt::Display for RootId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Component => "Root::Component",
                Self::Task => "Root::Task",
            }
        )
    }
}
