use crate::*;

#[derive(Debug, Default, Clone)]
pub struct TypeEntity {
    pub assigned: Option<Ty>,
    pub annotated: Option<Ty>,
    pub node: Uuid,
}

impl TypeEntity {
    pub fn new(node: Uuid, assigned: Option<Ty>, annotated: Option<Ty>) -> Self {
        Self {
            assigned,
            annotated,
            node,
        }
    }
    pub fn ty(&self) -> Option<&Ty> {
        self.assigned.as_ref().or_else(|| self.annotated.as_ref())
    }
}
