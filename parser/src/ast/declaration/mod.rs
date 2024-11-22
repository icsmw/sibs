mod conflict;
mod interest;

mod argument_declaration;
mod closure;
mod function_declaration;
mod variable_declaration;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;
use asttree::*;
use diagnostics::*;

impl AsVec<DeclarationId> for DeclarationId {
    fn as_vec() -> Vec<DeclarationId> {
        DeclarationId::as_vec()
    }
}

impl Read<Declaration, DeclarationId> for Declaration {}

impl TryRead<Declaration, DeclarationId> for Declaration {
    fn try_read(
        parser: &mut Parser,
        id: DeclarationId,
    ) -> Result<Option<Declaration>, LinkedErr<E>> {
        Ok(match id {
            DeclarationId::FunctionDeclaration => {
                FunctionDeclaration::read(parser)?.map(Declaration::FunctionDeclaration)
            }
            DeclarationId::VariableDeclaration => {
                VariableDeclaration::read(parser)?.map(Declaration::VariableDeclaration)
            }
            DeclarationId::ArgumentDeclaration => {
                ArgumentDeclaration::read(parser)?.map(Declaration::ArgumentDeclaration)
            }
            DeclarationId::VariableType => {
                VariableType::read(parser)?.map(Declaration::VariableType)
            }
            DeclarationId::VariableTypeDeclaration => {
                VariableTypeDeclaration::read(parser)?.map(Declaration::VariableTypeDeclaration)
            }
            DeclarationId::VariableVariants => {
                VariableVariants::read(parser)?.map(Declaration::VariableVariants)
            }
            DeclarationId::Closure => Closure::read(parser)?.map(Declaration::Closure),
        })
    }
}
