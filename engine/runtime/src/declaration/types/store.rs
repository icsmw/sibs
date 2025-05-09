use crate::*;

/// Represents a storage for types and scopes used in the analysis phase of a task execution.
///
/// # Fields
///
/// * `scopes` - A mapping of task scopes, where each `Uuid` corresponds to a unique task node (`Root::Task`).
/// * `location` - A stack that tracks the current analysis path within the node tree, representing the sequence of nodes being processed.
#[derive(Debug)]
pub struct TyStore {
    pub scopes: HashMap<Uuid, TyScope>,
    pub location: Vec<Uuid>,
}

impl TyStore {
    pub fn open(&mut self, uuid: &Uuid) -> Result<(), E> {
        self.scopes.entry(*uuid).or_default();
        self.location.push(*uuid);
        self.enter(uuid)
    }
    pub fn close(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.leave()?;
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveGlobalContext)
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
    pub fn get(&self) -> Result<&TyScope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get(sc).ok_or(E::FailToFindContext(*sc))
        } else {
            Err(E::NoRootContext)
        }
    }
    pub fn get_mut(&mut self) -> Result<&mut TyScope, E> {
        if let Some(sc) = self.location.last() {
            self.scopes.get_mut(sc).ok_or(E::FailToFindContext(*sc))
        } else {
            Err(E::NoRootContext)
        }
    }
    pub fn get_scope(&self, uuid: &Uuid) -> Option<&TyScope> {
        self.scopes.get(uuid)
    }
}

impl Default for TyStore {
    fn default() -> Self {
        let root = Uuid::new_v4();
        let mut scopes = HashMap::new();
        scopes.insert(root, TyScope::default());
        Self {
            scopes,
            location: vec![root],
        }
    }
}
