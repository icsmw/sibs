use crate::*;

#[derive(Debug, Default)]
pub struct TyParent {
    pub tys: HashMap<Uuid, Ty>,
}

impl TyParent {
    pub fn set(&mut self, uuid: &Uuid, ty: Ty) -> bool {
        if self.tys.contains_key(uuid) {
            false
        } else {
            self.tys.insert(*uuid, ty);
            true
        }
    }
    pub fn get(&self, uuid: &Uuid) -> Option<&Ty> {
        self.tys.get(uuid)
    }
    pub fn exist(&self, uuid: &Uuid) -> bool {
        self.tys.contains_key(uuid)
    }
}
