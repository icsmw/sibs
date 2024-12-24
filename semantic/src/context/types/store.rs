use crate::*;

#[derive(Debug)]
pub struct Types {
    pub scopes: HashMap<Uuid, Scope>,
    pub location: Vec<Uuid>,
}

impl Types {
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
    pub fn enter(&mut self, uuid: &Uuid) -> Result<(), E> {
        self.get_mut()?.enter(uuid);
        Ok(())
    }
    pub fn leave(&mut self) -> Result<(), E> {
        self.get_mut()?.leave()
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, edt: TypeEntity) -> Result<(), E> {
        self.get_mut()?.insert(name, edt)
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Result<Option<&TypeEntity>, E> {
        Ok(self.get()?.lookup(name))
    }
    pub fn get(&self) -> Result<&Scope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get(sc).ok_or(E::FailToFindScope(*sc))
        } else {
            Err(E::NoRootScope)
        }
    }
    pub fn get_mut(&mut self) -> Result<&mut Scope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get_mut(sc).ok_or(E::FailToFindScope(*sc))
        } else {
            Err(E::NoRootScope)
        }
    }
}

impl Default for Types {
    fn default() -> Self {
        let root = Uuid::new_v4();
        let mut scopes = HashMap::new();
        scopes.insert(root, Scope::default());
        Self {
            scopes,
            location: vec![root],
        }
    }
}
