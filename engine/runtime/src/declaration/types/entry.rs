use crate::*;

#[derive(Debug, Default, Clone)]
pub struct TypeEntity {
    pub assigned: Option<Ty>,
    pub annotated: Option<Ty>,
    /// Uuid of node, type belongs to
    pub node: Uuid,
    /// Position of type place
    pub position: Position,
}

impl TypeEntity {
    pub fn new(
        node: Uuid,
        position: Position,
        assigned: Option<Ty>,
        annotated: Option<Ty>,
    ) -> Self {
        Self {
            assigned,
            annotated,
            node,
            position,
        }
    }
    pub fn ty(&self) -> Option<&Ty> {
        self.assigned.as_ref().or_else(|| self.annotated.as_ref())
    }
}
