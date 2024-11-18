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
    /// #[...]
    Gatekeeper(Gatekeeper),
    /// skip([task_args], func())
    Skip(Skip),
}
