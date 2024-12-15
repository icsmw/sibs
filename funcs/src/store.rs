use crate::*;
use std::collections::HashMap;

pub struct FnStore {
    descs: HashMap<String, FnDesc>,
}

impl FnStore {
    pub fn insert<S: AsRef<str>>(&mut self, name: S, desc: FnDesc) -> Result<(), E> {
        let name = name.as_ref().to_string();
        if self.descs.contains_key(&name) {
            return Err(E::FuncAlreadyRegistered(name));
        }
        self.descs.insert(name, desc);
        Ok(())
    }
}
