use crate::*;

impl ConflictResolver<StatementId> for StatementId {
    fn resolve_conflict(&self, _id: &StatementId) -> StatementId {
        match self {
            Self::Break
            | Self::Return
            | Self::Each
            | Self::For
            | Self::Loop
            | Self::While
            | Self::Assignation
            | Self::Optional
            | Self::OneOf
            | Self::Join
            | Self::Block
            | Self::If => self.clone(),
        }
    }
}
