mod argument_declaration;
mod closure_declaration;
mod function_declaration;
mod include_declaration;
mod module_declaration;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

pub use argument_declaration::*;
pub use closure_declaration::*;
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
    /// str, num, Vec<str>, unknown etc.
    VariableType(VariableType),
    /// a: str, a: numr, a: string[], a: unknown etc.
    VariableTypeDeclaration(VariableTypeDeclaration),
    /// Name of variable, which is used only in declaration.
    VariableName(VariableName),
    /// ex: |a: num|: num
    ClosureDeclaration(ClosureDeclaration),
}

impl Identification for Declaration {
    fn uuid(&self) -> &Uuid {
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
            Self::ClosureDeclaration(n) => &n.uuid,
        }
    }
    fn ident(&self) -> String {
        match self {
            Self::IncludeDeclaration(..) => DeclarationId::IncludeDeclaration.to_string(),
            Self::ModuleDeclaration(..) => DeclarationId::ModuleDeclaration.to_string(),
            Self::ArgumentDeclaration(..) => DeclarationId::ArgumentDeclaration.to_string(),
            Self::FunctionDeclaration(..) => DeclarationId::FunctionDeclaration.to_string(),
            Self::VariableDeclaration(..) => DeclarationId::VariableDeclaration.to_string(),
            Self::VariableType(..) => DeclarationId::VariableType.to_string(),
            Self::VariableTypeDeclaration(..) => DeclarationId::VariableTypeDeclaration.to_string(),
            Self::VariableVariants(..) => DeclarationId::VariableVariants.to_string(),
            Self::VariableName(..) => DeclarationId::VariableName.to_string(),
            Self::ClosureDeclaration(..) => DeclarationId::ClosureDeclaration.to_string(),
        }
    }
}

impl Diagnostic for Declaration {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        match self {
            Self::IncludeDeclaration(n) => n.located(src, pos),
            Self::ModuleDeclaration(n) => n.located(src, pos),
            Self::ArgumentDeclaration(n) => n.located(src, pos),
            Self::FunctionDeclaration(n) => n.located(src, pos),
            Self::VariableDeclaration(n) => n.located(src, pos),
            Self::VariableType(n) => n.located(src, pos),
            Self::VariableTypeDeclaration(n) => n.located(src, pos),
            Self::VariableVariants(n) => n.located(src, pos),
            Self::VariableName(n) => n.located(src, pos),
            Self::ClosureDeclaration(n) => n.located(src, pos),
        }
    }
    fn get_position(&self) -> Position {
        match self {
            Self::IncludeDeclaration(n) => n.get_position(),
            Self::ModuleDeclaration(n) => n.get_position(),
            Self::ArgumentDeclaration(n) => n.get_position(),
            Self::FunctionDeclaration(n) => n.get_position(),
            Self::VariableDeclaration(n) => n.get_position(),
            Self::VariableType(n) => n.get_position(),
            Self::VariableTypeDeclaration(n) => n.get_position(),
            Self::VariableVariants(n) => n.get_position(),
            Self::VariableName(n) => n.get_position(),
            Self::ClosureDeclaration(n) => n.get_position(),
        }
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        match self {
            Self::IncludeDeclaration(n) => n.childs(),
            Self::ModuleDeclaration(n) => n.childs(),
            Self::ArgumentDeclaration(n) => n.childs(),
            Self::FunctionDeclaration(n) => n.childs(),
            Self::VariableDeclaration(n) => n.childs(),
            Self::VariableType(n) => n.childs(),
            Self::VariableTypeDeclaration(n) => n.childs(),
            Self::VariableVariants(n) => n.childs(),
            Self::VariableName(n) => n.childs(),
            Self::ClosureDeclaration(n) => n.childs(),
        }
    }
}

impl From<Declaration> for Node {
    fn from(val: Declaration) -> Self {
        Node::Declaration(val)
    }
}

impl<'a> Lookup<'a> for Declaration {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            Self::IncludeDeclaration(n) => n.lookup(trgs),
            Self::ModuleDeclaration(n) => n.lookup(trgs),
            Self::ArgumentDeclaration(n) => n.lookup(trgs),
            Self::FunctionDeclaration(n) => n.lookup(trgs),
            Self::VariableDeclaration(n) => n.lookup(trgs),
            Self::VariableType(n) => n.lookup(trgs),
            Self::VariableTypeDeclaration(n) => n.lookup(trgs),
            Self::VariableVariants(n) => n.lookup(trgs),
            Self::VariableName(n) => n.lookup(trgs),
            Self::ClosureDeclaration(n) => n.lookup(trgs),
        }
    }
}

impl FindMutByUuid for Declaration {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        match self {
            Self::IncludeDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::ModuleDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::ArgumentDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::FunctionDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::VariableDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::VariableType(n) => n.find_mut_by_uuid(uuid),
            Self::VariableTypeDeclaration(n) => n.find_mut_by_uuid(uuid),
            Self::VariableVariants(n) => n.find_mut_by_uuid(uuid),
            Self::VariableName(n) => n.find_mut_by_uuid(uuid),
            Self::ClosureDeclaration(n) => n.find_mut_by_uuid(uuid),
        }
    }
}

impl SrcLinking for Declaration {
    fn link(&self) -> SrcLink {
        match self {
            Self::IncludeDeclaration(n) => n.link(),
            Self::ModuleDeclaration(n) => n.link(),
            Self::ArgumentDeclaration(n) => n.link(),
            Self::FunctionDeclaration(n) => n.link(),
            Self::VariableDeclaration(n) => n.link(),
            Self::VariableType(n) => n.link(),
            Self::VariableTypeDeclaration(n) => n.link(),
            Self::VariableVariants(n) => n.link(),
            Self::VariableName(n) => n.link(),
            Self::ClosureDeclaration(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}
