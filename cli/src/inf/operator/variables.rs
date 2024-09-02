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
        map.set(name, ty)?;
        Ok(())
    }
    pub fn withdraw(&mut self, owner: &Uuid) -> Result<LocalVariablesMap, E> {
        self.map.remove(owner).ok_or(E::ComponentNotFound(*owner))
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            operator::{ExpectedValueType, E},
            Configuration, Context, GlobalVariablesMap, Journal, Scope,
        },
        process_string,
        reader::{Reader, Sources},
    };

    #[tokio::test]
    async fn success() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/verification/success.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |mut components: Vec<Element>, cx: Context, _sc: Scope, _: Journal| async move {
                let mut variables = GlobalVariablesMap::default();
                for component in components.iter() {
                    component
                        .linking(&mut variables, component, &components, &cx)
                        .await
                        .expect("linking variables is done");
                }
                for component in components.iter_mut() {
                    component
                        .as_mut_component()?
                        .link(&mut variables)
                        .expect("component linked");
                }
                for component in components.iter() {
                    component
                        .varification(component, &components, &cx)
                        .await
                        .expect("component varified");
                }
                assert_eq!(components.len(), 4);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn fail() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/verification/fail.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |mut components: Vec<Element>, cx: Context, _sc: Scope, _: Journal| async move {
                let mut variables = GlobalVariablesMap::default();
                for component in components.iter() {
                    component
                        .linking(&mut variables, component, &components, &cx)
                        .await
                        .expect("linking variables is done");
                }
                for component in components.iter_mut() {
                    component
                        .as_mut_component()?
                        .link(&mut variables)
                        .expect("linking variables is done");
                }
                for component in components.iter() {
                    component
                        .expected(component, &components, &cx)
                        .await
                        .expect("linking variables is done");
                }
                for component in components.iter() {
                    let result = component.varification(component, &components, &cx).await;
                    if let Err(err) = result.as_ref() {
                        cx.atlas.report_err(err).await.expect("report created");
                    }
                    assert!(result.is_err());
                }
                assert_eq!(components.len(), 4);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
