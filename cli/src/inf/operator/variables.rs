use uuid::Uuid;

use crate::inf::{operator::E, ValueRef};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct LocalVariablesMap {
    pub map: HashMap<String, ValueRef>,
}

impl LocalVariablesMap {
    pub fn set<S: AsRef<str>>(&mut self, name: S, ty: ValueRef) -> Result<(), E> {
        if self.map.contains_key(name.as_ref()) {
            return Err(E::MultipleDeclaration(name.as_ref().to_string()));
        }
        self.map.insert(name.as_ref().to_string(), ty);
        Ok(())
    }
    pub fn get<S: AsRef<str>>(&self, name: S) -> Result<ValueRef, E> {
        self.map
            .get(name.as_ref())
            .ok_or(E::VariableIsNotDeclared(name.as_ref().to_string()))
            .cloned()
    }
    pub fn contains<S: AsRef<str>>(&self, name: S) -> bool {
        self.map.contains_key(name.as_ref())
    }
}

#[derive(Debug, Default, Clone)]
pub struct GlobalVariablesMap {
    pub map: HashMap<Uuid, LocalVariablesMap>,
}

impl GlobalVariablesMap {
    pub fn set<S: AsRef<str>>(&mut self, owner: &Uuid, name: S, ty: ValueRef) -> Result<(), E> {
        let map = self.map.entry(*owner).or_default();
        if map.contains(name.as_ref()) {
            return Err(E::MultipleDeclaration(name.as_ref().to_string()));
        }
        map.set(name.as_ref().to_string(), ty)?;
        Ok(())
    }
    pub fn get<S: AsRef<str>>(&self, owner: &Uuid, name: S) -> Result<ValueRef, E> {
        let map = self
            .map
            .get(owner)
            .ok_or(E::NoOwnerForVariable(name.as_ref().to_string()))?;
        map.get(name.as_ref())
    }
}

// #[cfg(test)]
// mod processing {
//     use crate::{
//         elements::Component,
//         error::LinkedErr,
//         inf::{
//             operator::{ExpectedValueType, E},
//             Configuration, Context, Journal, Scope,
//         },
//         process_string,
//         reader::{Dissect, Reader, Sources},
//     };

//     #[tokio::test]
//     async fn reading() {
//         process_string!(
//             &Configuration::logs(false),
//             &include_str!("../../tests/verification/success.sibs"),
//             |reader: &mut Reader, src: &mut Sources| {
//                 let mut components: Vec<Component> = Vec::new();
//                 while let Some(task) = src.report_err_if(Component::dissect(reader))? {
//                     components.push(task);
//                 }
//                 Ok::<Vec<Component>, LinkedErr<E>>(components)
//             },
//             |mut components: Vec<Component>, _cx: Context, _sc: Scope, _: Journal| async move {
//                 for (i, mut component) in components.iter_mut().enumerate() {
//                     component
//                         .linking(&mut component, &components)
//                         .expect("component returns some value");
//                 }
//                 Ok::<(), LinkedErr<E>>(())
//             }
//         );
//     }
// }
