use crate::*;

#[derive(Debug, Default, Clone)]
pub struct TypeEntity {
    pub assigned: Option<Ty>,
    pub annotated: Option<Ty>,
}

impl TypeEntity {
    pub fn new(assigned: Option<Ty>, annotated: Option<Ty>) -> Self {
        Self {
            assigned,
            annotated,
        }
    }
}
