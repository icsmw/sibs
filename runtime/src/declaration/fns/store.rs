use crate::*;

#[derive(Debug, Default)]
pub struct Fns {
    pub efns: EFns,
    pub ufns: UFns,
}

impl Fns {
    pub fn lookup<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(entity) = self.ufns.lookup(name.as_ref(), caller) {
            Some(FnEntity::UFn(entity))
        } else if let Some(entity) = self.efns.lookup(name, caller) {
            Some(FnEntity::EFn(entity))
        } else {
            None
        }
    }
    pub fn lookup_by_caller(&self, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(name) = self.ufns.links.get(caller) {
            self.ufns.funcs.get(name).map(FnEntity::UFn)
        } else if let Some(name) = self.efns.links.get(caller) {
            self.efns.funcs.get(name).map(FnEntity::EFn)
        } else {
            None
        }
    }
}
