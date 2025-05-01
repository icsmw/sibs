mod conflict;

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

use crate::*;

impl AsVec<DeclarationId> for DeclarationId {
    fn as_vec() -> Vec<DeclarationId> {
        DeclarationId::as_vec()
    }
}

impl TryRead<Declaration, DeclarationId> for Declaration {
    fn try_read(
        parser: &Parser,
        id: DeclarationId,
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            DeclarationId::ModuleDeclaration => ModuleDeclaration::read_as_linked(parser)?,
            DeclarationId::IncludeDeclaration => IncludeDeclaration::read_as_linked(parser)?,
            DeclarationId::FunctionDeclaration => FunctionDeclaration::read_as_linked(parser)?,
            DeclarationId::VariableDeclaration => VariableDeclaration::read_as_linked(parser)?,
            DeclarationId::ArgumentDeclaration => ArgumentDeclaration::read_as_linked(parser)?,
            DeclarationId::VariableType => VariableType::read_as_linked(parser)?,
            DeclarationId::VariableTypeDeclaration => {
                VariableTypeDeclaration::read_as_linked(parser)?
            }
            DeclarationId::VariableVariants => VariableVariants::read_as_linked(parser)?,
            DeclarationId::VariableName => VariableName::read_as_linked(parser)?,
            DeclarationId::ClosureDeclaration => ClosureDeclaration::read_as_linked(parser)?,
        })
    }
}
