use uuid::Uuid;

use crate::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeContext {
    pub scopes: HashMap<Uuid, HashMap<String, DataType>>,
    pub location: Vec<Uuid>,
    pub parent: Option<DataType>,
    // pub annotations: Annotations,
}

impl TypeContext {
    pub fn set_parent_ty(&mut self, ty: DataType) {
        self.parent = Some(ty);
    }
    pub fn get_parent_ty(&self) -> Option<&DataType> {
        self.parent.as_ref()
    }
    pub fn drop_parent_ty(&mut self) {
        self.parent = None;
    }
    // pub fn annotate(&mut self, uuid: &Uuid, dt: DataType) {
    //     self.annotations.add(uuid, dt);
    // }

    // pub fn get_annotation(&mut self, uuid: &Uuid) -> Option<&DataType> {
    //     self.annotations.lookup(uuid)
    // }

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

    pub fn insert<S: AsRef<str>>(&mut self, name: S, dt: DataType) -> Result<(), E> {
        if let Some(uuid) = self.location.last() {
            if let Some(sc) = self.scopes.get_mut(uuid) {
                sc.insert(name.as_ref().to_owned(), dt);
                return Ok(());
            }
        }
        Err(E::NoCurrentScope)
    }

    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<&DataType> {
        for uuid in self.location.iter().rev() {
            if let Some(dt) = self.scopes.get(uuid)?.get(name.as_ref()) {
                return Some(dt);
            }
        }
        None
    }
}
