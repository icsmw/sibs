mod conflict;
mod interest;
mod read;

mod closure;
mod variable_declaration;
mod variable_type;
mod variable_variants;

pub use closure::*;
pub use variable_declaration::*;
pub use variable_type::*;
pub use variable_variants::*;

use std::fmt;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone")]
#[derive(Debug, Clone)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    VariableVariants(VariableVariants),
    VariableType(VariableType),
    Closure(Closure),
}

impl fmt::Display for DeclarationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::VariableDeclaration => "Declaration::VariableDeclaration",
                Self::VariableVariants => "Declaration::VariableVariants",
                Self::VariableType => "Declaration::VariableType",
                Self::Closure => "Declaration::Closure",
            }
        )
    }
}
