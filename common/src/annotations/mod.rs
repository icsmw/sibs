use crate::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct Annotations {
    pub types: HashMap<Uuid, DataType>,
}

impl Annotations {
    pub fn add(&mut self, uuid: &Uuid, dt: DataType) {
        self.types.insert(*uuid, dt);
    }
    pub fn lookup(&self, uuid: &Uuid) -> Option<&DataType> {
        self.types.get(uuid)
    }
}
