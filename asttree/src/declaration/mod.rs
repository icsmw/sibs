mod argument_declaration;
mod function_declaration;
mod include_declaration;
mod module_declaration;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

pub use argument_declaration::*;
pub use function_declaration::*;
pub use include_declaration::*;
pub use module_declaration::*;
pub use variable_declaration::*;
pub use variable_name::*;
pub use variable_type::*;
pub use variable_type_declaration::*;
pub use variable_variants::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
pub enum Declaration {
    /// include "path_to_scenario"
    IncludeDeclaration(IncludeDeclaration),
    /// mod "path_to_module"
    ModuleDeclaration(ModuleDeclaration),
    /// fn name() { ... }; fn name(a, b) { ... }; etc.
    FunctionDeclaration(FunctionDeclaration),
    /// let a = 5; etc.
    VariableDeclaration(VariableDeclaration),
    /// a: string, a: number, a: string[], a: unknown, a: 1 | 2 | 3, a: "one" | "two" etc.
    ArgumentDeclaration(ArgumentDeclaration),
    /// a: "one" | "two", a: 1 | 2 etc.
    VariableVariants(VariableVariants),
    /// string, number, Vec<string>, unknown etc.
    VariableType(VariableType),
    /// a: string, a: number, a: string[], a: unknown etc.
    VariableTypeDeclaration(VariableTypeDeclaration),
    /// Name of variable, which is used only in declaration.
    VariableName(VariableName),
}

impl Declaration {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::IncludeDeclaration(n) => &n.uuid,
            Self::ModuleDeclaration(n) => &n.uuid,
            Self::ArgumentDeclaration(n) => &n.uuid,
            Self::FunctionDeclaration(n) => &n.uuid,
            Self::VariableDeclaration(n) => &n.uuid,
            Self::VariableType(n) => &n.uuid,
            Self::VariableTypeDeclaration(n) => &n.uuid,
            Self::VariableVariants(n) => &n.uuid,
            Self::VariableName(n) => &n.uuid,
        }
    }
}

impl From<Declaration> for Node {
    fn from(val: Declaration) -> Self {
        Node::Declaration(val)
    }
}
