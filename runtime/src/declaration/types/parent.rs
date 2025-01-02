use crate::*;

#[derive(Debug, Default)]
pub struct TyParent {
    pub ty: Option<Ty>,
}

impl TyParent {
    pub fn set(&mut self, ty: Ty) {
        self.ty = Some(ty);
    }
    pub fn withdraw(&mut self) -> Option<Ty> {
        self.ty.take()
    }
    pub fn is_empty(&self) -> bool {
        self.ty.is_none()
    }
    pub fn drop(&mut self) {
        self.ty = None;
    }
}
