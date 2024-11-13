mod conflict;
mod interest;
mod read;

mod argument_declaration;
mod closure;
mod variable_declaration;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

pub use argument_declaration::*;
pub use closure::*;
pub use variable_declaration::*;
pub use variable_type::*;
pub use variable_type_declaration::*;
pub use variable_variants::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Declaration {
    /// let a = 5; etc.
    VariableDeclaration(VariableDeclaration),
    /// a: string, a: number, a: string[], a: unknown etc.
    ArgumentDeclaration(ArgumentDeclaration),
    /// a: "one" | "two", a: 1 | 2 etc.
    VariableVariants(VariableVariants),
    /// string, number, Vec<string>, unknown etc.
    VariableType(VariableType),
    /// a: string, a: number, a: string[], a: unknown etc.
    VariableTypeDeclaration(VariableTypeDeclaration),
    /// () { ... }; (a, b) { ... }; etc.
    Closure(Closure),
}
