use crate::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeContext {
    pub scopes: Vec<HashMap<String, DataType>>,
}

impl TypeContext {
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

    pub fn lookup(&self, name: &str) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(dt) = scope.get(name) {
                return Some(dt);
            }
        }
        None
    }
}
