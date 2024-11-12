use crate::*;

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
        nodes: &Nodes,
    ) -> Result<Option<Declaration>, E> {
        Ok(match id {
            DeclarationId::VariableDeclaration => {
                VariableDeclaration::read(parser, nodes)?.map(Declaration::VariableDeclaration)
            }
            DeclarationId::VariableType => {
                VariableType::read(parser, nodes)?.map(Declaration::VariableType)
            }
            DeclarationId::VariableTypeDeclaration => VariableTypeDeclaration::read(parser, nodes)?
                .map(Declaration::VariableTypeDeclaration),
            DeclarationId::VariableVariants => {
                VariableVariants::read(parser, nodes)?.map(Declaration::VariableVariants)
            }
            DeclarationId::Closure => Closure::read(parser, nodes)?.map(Declaration::Closure),
        })
    }
}
