use crate::*;

#[derive(Debug, Default)]
pub struct TypesTable {
    pub tys: HashMap<Uuid, DataType>,
}

impl TypesTable {
    pub fn get(&self, uuid: &Uuid) -> Option<DataType> {
        self.tys.get(uuid).cloned()
    }
    pub fn set(&mut self, uuid: &Uuid, ty: DataType) {
        self.tys.insert(*uuid, ty);
    }
    pub fn has(&self, uuid: &Uuid) -> bool {
        self.tys.contains_key(uuid)
    }
}
