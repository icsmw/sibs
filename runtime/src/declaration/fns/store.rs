use crate::*;

#[derive(Debug, Default)]
pub struct Fns {
    pub efns: EFns,
    pub ufns: UFns,
    pub cfns: CFns,
}

impl Fns {
    pub fn lookup<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(entity) = self.ufns.lookup(name.as_ref(), caller) {
            Some(FnEntity::UFn(entity))
        } else {
            self.efns.lookup(name, caller).map(FnEntity::EFn)
        }
    }
    pub fn lookup_by_caller(&self, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(name) = self.ufns.links.get(caller) {
            self.ufns.funcs.get(name).map(FnEntity::UFn)
        } else if let Some(name) = self.efns.links.get(caller) {
            self.efns.funcs.get(name).map(FnEntity::EFn)
        } else if let Some(uuid) = self.cfns.links.get(caller) {
            self.cfns.funcs.get(uuid).map(FnEntity::CFn)
        } else {
            None
        }
    }
    pub fn lookup_by_uuid(&mut self, uuid: &Uuid, caller: &Uuid) -> Option<FnEntity<'_>> {
        self.cfns.lookup(uuid, caller).map(FnEntity::CFn)
    }
}
