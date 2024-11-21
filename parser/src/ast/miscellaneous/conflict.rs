use crate::*;
use asttree::*;

impl ConflictResolver<MiscellaneousId> for MiscellaneousId {
    fn resolve_conflict(&self, _id: &MiscellaneousId) -> MiscellaneousId {
        match self {
            Self::Comment | Self::Meta | Self::Include | Self::Module => self.clone(),
        }
    }
}
