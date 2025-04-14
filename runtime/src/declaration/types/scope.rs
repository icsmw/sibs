use crate::*;

#[derive(Debug, Default)]
pub struct TyScope {
    pub levels: HashMap<Uuid, HashMap<String, TypeEntity>>,
    pub location: Vec<Uuid>,
    pub parent: TyParent,
}

impl TyScope {
    pub fn enter(&mut self, uuid: &Uuid) {
        self.levels.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn leave(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveRootContextLevel)
        }
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, edt: TypeEntity) -> Result<(), E> {
        if let Some(uuid) = self.location.last() {
            if let Some(sc) = self.levels.get_mut(uuid) {
                sc.insert(name.as_ref().to_owned(), edt);
                return Ok(());
            }
        }
        Err(E::NoCurrentContextLevel)
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<&TypeEntity> {
        for uuid in self.location.iter().rev() {
            if let Some(edt) = self.levels.get(uuid)?.get(name.as_ref()) {
                return Some(edt);
            }
        }
        None
    }
}
