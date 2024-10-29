use crate::*;

impl ConflictResolver<ExpressionId> for ExpressionId {
    fn resolve_conflict(&self, _id: &ExpressionId) -> ExpressionId {
        match self {
            Self::Variable => self.clone(),
        }
    }
}
