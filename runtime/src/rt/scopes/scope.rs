use std::sync::Arc;

use crate::*;

#[derive(Debug, Default)]
pub struct VlScope {
    pub levels: HashMap<Uuid, HashMap<String, Arc<RtValue>>>,
    pub location: Vec<Uuid>,
    pub parent: RtParent,
}

impl VlScope {
    pub fn enter(&mut self, uuid: &Uuid) {
        self.levels.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn leave(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveRootScopeLevel)
        }
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, vl: RtValue) -> Result<(), E> {
        if let Some(uuid) = self.location.last() {
            if let Some(sc) = self.levels.get_mut(uuid) {
                sc.insert(name.as_ref().to_owned(), Arc::new(vl));
                return Ok(());
            }
        }
        Err(E::NoCurrentScopeLevel)
    }
    pub fn update<S: AsRef<str>>(&mut self, name: S, vl: RtValue) -> Result<(), E> {
        for uuid in self.location.iter().rev() {
            let scope = self.levels.get_mut(uuid).ok_or(E::NoCurrentScopeLevel)?;
            if scope.contains_key(name.as_ref()) {
                scope.insert(name.as_ref().to_owned(), Arc::new(vl));
                return Ok(());
            }
        }
        Err(E::VariableNotFound(name.as_ref().to_owned()))
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<Arc<RtValue>> {
        for uuid in self.location.iter().rev() {
            if let Some(vl) = self.levels.get(uuid)?.get(name.as_ref()) {
                return Some(vl.clone());
            }
        }
        None
    }
}
