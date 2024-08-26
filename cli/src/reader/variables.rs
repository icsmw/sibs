use crate::{inf::ValueRef, reader::E};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Variables {
    pub map: HashMap<String, ValueRef>,
}

impl Variables {
    pub fn drop(&mut self) {
        self.map.clear();
    }
    pub fn set<S: AsRef<str>>(&mut self, name: S, ty: ValueRef) -> Result<(), E> {
        if self.map.contains_key(name.as_ref()) {
            return Err(E::MultipleDeclaration(name.as_ref().to_string()));
        }
        self.map.insert(name.as_ref().to_string(), ty);
        Ok(())
    }
}
