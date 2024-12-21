use crate::*;

#[derive(Debug, Default, Clone)]
pub struct EntityType {
    pub assigned: Option<DataType>,
    pub annotated: Option<DataType>,
}

impl EntityType {
    pub fn new(assigned: Option<DataType>, annotated: Option<DataType>) -> Self {
        Self {
            assigned,
            annotated,
        }
    }
}
