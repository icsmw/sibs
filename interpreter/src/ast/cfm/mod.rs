mod conflict;
mod interest;
mod read;

mod gatekeeper;

pub use gatekeeper::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum ControlFlowModifier {
    Gatekeeper(Gatekeeper),
}
