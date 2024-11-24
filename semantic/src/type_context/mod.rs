use uuid::Uuid;

use crate::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TypeContext {
    pub scopes: Vec<HashMap<String, DataType>>,
    pub annotations: Annotations,
}

impl TypeContext {
    pub fn annotate(&mut self, uuid: &Uuid, dt: DataType) {
        self.annotations.add(uuid, dt);
    }

    pub fn get_annotation(&mut self, uuid: &Uuid) -> Option<&DataType> {
        self.annotations.lookup(uuid)
    }

    pub fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn leave(&mut self) -> Result<(), E> {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveGlobalScope)
        }
    }

    pub fn insert(&mut self, name: String, dt: DataType) -> Result<(), E> {
        if let Some(sc) = self.scopes.last_mut() {
            sc.insert(name, dt);
            Ok(())
        } else {
            Err(E::NoCurrentScope)
        }
    }

    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(dt) = scope.get(name.as_ref()) {
                return Some(dt);
            }
        }
        None
    }
}
