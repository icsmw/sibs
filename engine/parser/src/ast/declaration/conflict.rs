use crate::*;

impl ConflictResolver<DeclarationId> for DeclarationId {
    fn resolve_conflict(&self, id: &DeclarationId) -> DeclarationId {
        match self {
            Self::GlobalsImport
            | Self::EnvsImport
            | Self::ModuleImport
            | Self::IncludeDeclaration
            | Self::GlobalDeclaration
            | Self::EnvDeclaration
            | Self::VariableDeclaration
            | Self::ArgumentDeclaration
            | Self::FunctionDeclaration
            | Self::VariableType
            | Self::VariableTypeDeclaration
            | Self::VariableVariants
            | Self::ClosureDeclaration => self.clone(),
            Self::VariableName => {
                if matches!(id, DeclarationId::ArgumentDeclaration) {
                    id.clone()
                } else {
                    self.clone()
                }
            }
        }
    }
}
