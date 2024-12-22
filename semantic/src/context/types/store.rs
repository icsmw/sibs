use crate::*;

#[derive(Debug, Default)]
pub struct Types {
    pub scopes: HashMap<Uuid, HashMap<String, TypeEntity>>,
    pub location: Vec<Uuid>,
    pub parent: Parent,
}

impl Types {
    pub fn enter(&mut self, uuid: &Uuid) {
        self.scopes.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn leave(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveGlobalScope)
        }
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, edt: TypeEntity) -> Result<(), E> {
        if let Some(uuid) = self.location.last() {
            if let Some(sc) = self.scopes.get_mut(uuid) {
                sc.insert(name.as_ref().to_owned(), edt);
                return Ok(());
            }
        }
        Err(E::NoCurrentScope)
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<&TypeEntity> {
        for uuid in self.location.iter().rev() {
            if let Some(edt) = self.scopes.get(uuid)?.get(name.as_ref()) {
                return Some(edt);
            }
        }
        None
    }
}
