use crate::*;

impl ConflictResolver<RootId> for RootId {
    fn resolve_conflict(&self, _id: &RootId) -> RootId {
        match self {
            Self::Component | Self::Task => self.clone(),
        }
    }
}
