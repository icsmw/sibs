use crate::*;

impl AsVec<DeclarationId> for DeclarationId {
    fn as_vec() -> Vec<DeclarationId> {
        DeclarationId::as_vec()
    }
}

impl Read<Declaration, DeclarationId> for Declaration {}

impl TryRead<Declaration, DeclarationId> for Declaration {
    fn try_read(parser: &mut Parser, id: DeclarationId) -> Result<Option<Declaration>, E> {
        Ok(match id {
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
