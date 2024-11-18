mod conflict;
mod interest;
mod link;
mod read;

mod component;
mod task;

pub use component::*;
pub use task::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Root {
    /// component name() { ... }, component name(pwd) { ... }
    Component(Component),
    /// task name() { ... }, private task name(arg: string, ...) { ... }
    Task(Task),
}
