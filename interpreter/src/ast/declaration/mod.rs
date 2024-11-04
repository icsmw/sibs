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

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Declaration {
    VariableDeclaration(VariableDeclaration),
    VariableVariants(VariableVariants),
    VariableType(VariableType),
    Closure(Closure),
}
