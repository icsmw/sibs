mod conflict;
mod interest;
mod read;

mod component;
mod task;

pub use component::*;
pub use task::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Root {
    Component(Component),
    Task(Task),
}
