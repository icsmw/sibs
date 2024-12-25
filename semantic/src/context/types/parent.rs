use crate::*;

#[derive(Debug, Default)]
pub struct Parent {
    pub ty: Option<DataType>,
}

impl Parent {
    pub fn set(&mut self, ty: DataType) {
        self.ty = Some(ty);
    }
    pub fn withdraw(&mut self) -> Option<DataType> {
        self.ty.take()
    }
    pub fn is_empty(&self) -> bool {
        self.ty.is_none()
    }
    pub fn drop(&mut self) {
        self.ty = None;
    }
}
