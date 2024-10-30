use crate::*;

impl ConflictResolver<ValueId> for ValueId {
    fn resolve_conflict(&self, _id: &ValueId) -> ValueId {
        match self {
            Self::PrimitiveString => self.clone(),
            Self::InterpolatedString => self.clone(),
            Self::Number => self.clone(),
            Self::Boolean => self.clone(),
            Self::Array => self.clone(),
            Self::Error => self.clone(),
        }
    }
}
