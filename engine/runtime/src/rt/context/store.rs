use std::sync::Arc;

use crate::*;

#[derive(Debug)]
pub struct Store {
    pub(crate) scopes: HashMap<Uuid, VlContext>,
    pub(crate) location: Vec<Uuid>,
    pub(crate) breaks: HashSet<Uuid>,
    pub(crate) loops: Vec<Uuid>,
    pub(crate) rcx: Vec<Uuid>,
    pub(crate) returns: HashMap<Uuid, RtValue>,
    pub(crate) cwd: PathBuf,
}

impl Store {
    pub fn new(cwd: PathBuf) -> Self {
        let root = Uuid::new_v4();
        let mut scopes = HashMap::new();
        scopes.insert(root, VlContext::default());
        Self {
            scopes,
            location: vec![root],
            loops: Vec::new(),
            breaks: HashSet::new(),
            rcx: Vec::new(),
            returns: HashMap::new(),
            cwd,
        }
    }
    pub fn open(&mut self, uuid: &Uuid) {
        self.scopes.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn close(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveGlobalContext)
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
        self.get_mut()?.parent.clear();
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
    fn get(&self) -> Result<&VlContext, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get(sc).ok_or(E::FailToFindContext(*sc))
        } else {
            Err(E::NoRootContext)
        }
    }
    fn get_mut(&mut self) -> Result<&mut VlContext, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get_mut(sc).ok_or(E::FailToFindContext(*sc))
        } else {
            Err(E::NoRootContext)
        }
    }
}
