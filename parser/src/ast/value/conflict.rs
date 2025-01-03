use crate::*;

impl ConflictResolver<ValueId> for ValueId {
    fn resolve_conflict(&self, _id: &ValueId) -> ValueId {
        match self {
            Self::PrimitiveString
            | Self::InterpolatedString
            | Self::Number
            | Self::Boolean
            | Self::Array
            | Self::Error
            | Self::Closure => self.clone(),
        }
    }
}
