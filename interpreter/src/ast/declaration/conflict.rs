use crate::*;

impl ConflictResolver<DeclarationId> for DeclarationId {
    fn resolve_conflict(&self, _id: &DeclarationId) -> DeclarationId {
        // Variable and Comparing are in conflict
        match self {
            Self::VariableDeclaration
            | Self::ArgumentDeclaration
            | Self::VariableType
            | Self::VariableTypeDeclaration
            | Self::VariableVariants
            | Self::Closure => self.clone(),
        }
    }
}
