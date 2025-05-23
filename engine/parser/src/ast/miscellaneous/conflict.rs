use crate::*;

impl ConflictResolver<MiscellaneousId> for MiscellaneousId {
    fn resolve_conflict(&self, _id: &MiscellaneousId) -> MiscellaneousId {
        match self {
            Self::Comment | Self::Meta => self.clone(),
        }
    }
}
