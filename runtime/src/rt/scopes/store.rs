use std::sync::Arc;

use crate::*;

#[derive(Debug)]
pub struct VlScopes {
    pub scopes: HashMap<Uuid, VlScope>,
    pub location: Vec<Uuid>,
}

impl VlScopes {
    pub fn open(&mut self, uuid: &Uuid) {
        self.scopes.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn close(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveGlobalScope)
        }
    }
    pub fn set_parent_vl(&mut self, vl: ParentValue) -> Result<(), E> {
        self.get_mut()?.parent.set(vl);
        Ok(())
    }
    pub fn withdraw_parent_vl(&mut self) -> Result<Option<ParentValue>, E> {
        Ok(self.get_mut()?.parent.withdraw())
    }
    pub fn drop_parent_vl(&mut self) -> Result<(), E> {
        self.get_mut()?.parent.drop();
        Ok(())
    }
    pub fn enter(&mut self, uuid: &Uuid) -> Result<(), E> {
        self.get_mut()?.enter(uuid);
        Ok(())
    }
    pub fn leave(&mut self) -> Result<(), E> {
        self.get_mut()?.leave()
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, vl: RtValue) -> Result<(), E> {
        self.get_mut()?.insert(name, vl)
    }
    pub fn update<S: AsRef<str>>(&mut self, name: S, vl: RtValue) -> Result<(), E> {
        self.get_mut()?.update(name, vl)
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Result<Option<Arc<RtValue>>, E> {
        Ok(self.get()?.lookup(name))
    }
    fn get(&self) -> Result<&VlScope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get(sc).ok_or(E::FailToFindScope(*sc))
        } else {
            Err(E::NoRootScope)
        }
    }
    fn get_mut(&mut self) -> Result<&mut VlScope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get_mut(sc).ok_or(E::FailToFindScope(*sc))
        } else {
            Err(E::NoRootScope)
        }
    }
}

impl Default for VlScopes {
    fn default() -> Self {
        let root = Uuid::new_v4();
        let mut scopes = HashMap::new();
        scopes.insert(root, VlScope::default());
        Self {
            scopes,
            location: vec![root],
        }
    }
}
