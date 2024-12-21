use crate::*;

#[derive(Debug, Default)]
pub struct Parent {
    pub ty: Option<DataType>,
}

impl Parent {
    pub fn set(&mut self, ty: DataType) {
        self.ty = Some(ty);
    }
    pub fn get(&self) -> Option<&DataType> {
        self.ty.as_ref()
    }
    pub fn drop(&mut self) {
        self.ty = None;
    }
}
