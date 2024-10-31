mod conflict;
mod interest;
mod read;

mod gatekeeper;

pub use gatekeeper::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum ControlFlowModifier {
    Gatekeeper(Gatekeeper),
}

impl fmt::Display for ControlFlowModifierId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Gatekeeper => "ControlFlowModifier::Gatekeeper",
            }
        )
    }
}
