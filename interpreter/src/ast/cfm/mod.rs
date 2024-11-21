mod conflict;
mod interest;
mod link;
mod read;

mod gatekeeper;
mod skip;

pub use gatekeeper::*;
pub use skip::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum ControlFlowModifier {
    /// #[skip([task_args], func())]
    /// #[skip([1, 2], func())]
    /// #[skip(["test", *], func())]
    /// #[skip([*,*], func())]
    Gatekeeper(Gatekeeper),
    /// skip([task_args], func())
    Skip(Skip),
}
