use uuid::Uuid;

use crate::inf::{operator::E, ValueRef};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ComponentlVariablesMap {
    pub map: HashMap<String, ValueRef>,
}

impl ComponentlVariablesMap {
    pub fn set<S: AsRef<str>>(&mut self, name: S, ty: ValueRef) -> Result<(), E> {
        self.map.insert(name.as_ref().to_string(), ty);
        Ok(())
    }
    pub fn get<S: AsRef<str>>(&self, name: S) -> Result<ValueRef, E> {
        self.map
            .get(name.as_ref())
            .ok_or(E::VariableIsNotDeclared(name.as_ref().to_string()))
            .cloned()
    }
}

#[derive(Debug, Default, Clone)]
pub struct VariablesMap {
    pub map: HashMap<Uuid, ComponentlVariablesMap>,
}

impl VariablesMap {
    pub fn set<S: AsRef<str>>(&mut self, owner: &Uuid, name: S, ty: ValueRef) -> Result<(), E> {
        self.map.entry(*owner).or_default().set(name, ty)?;
        Ok(())
    }
    pub fn get<S: AsRef<str>>(&mut self, owner: &Uuid, name: S) -> Result<ValueRef, E> {
        self.map
            .get(owner)
            .ok_or(E::UnknownComponent(owner.to_owned()))?
            .get(name.as_ref())
    }
}
