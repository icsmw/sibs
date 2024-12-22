use crate::*;

#[derive(Debug, Default, Clone)]
pub struct TypeEntity {
    pub assigned: Option<DataType>,
    pub annotated: Option<DataType>,
}

impl TypeEntity {
    pub fn new(assigned: Option<DataType>, annotated: Option<DataType>) -> Self {
        Self {
            assigned,
            annotated,
        }
    }
}
