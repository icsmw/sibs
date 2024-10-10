mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{
    elements::{Element, SimpleString, Task},
    inf::{scenario, Context},
};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Component {
    pub cwd: Option<PathBuf>,
    pub name: SimpleString,
    pub elements: Vec<Element>,
    pub token: usize,
    pub uuid: Uuid,
}

impl Component {
    pub fn get_name(&self) -> String {
        self.name.value.to_owned()
    }
    pub fn get_task(&self, name: &str) -> Option<(&Element, Vec<&Element>)> {
        let mut gatekeepers = Vec::new();
        for el in self.elements.iter() {
            if let Element::Task(task, _) = &el {
                if task.get_name() == name {
                    return Some((el, gatekeepers));
                } else {
                    gatekeepers.clear();
                }
            } else if matches!(el, Element::Gatekeeper(..)) {
                gatekeepers.push(el);
            }
        }
        None
    }
    pub fn get_cwd(&self, cx: &Context) -> Result<Option<PathBuf>, scenario::E> {
        Ok(if let Some(path) = self.cwd.as_ref() {
            Some(cx.scenario.to_abs_path(path)?)
        } else {
            None
        })
    }
    pub fn get_tasks(&self) -> Vec<&Task> {
        self.elements
            .iter()
            .filter_map(|el| {
                if let Element::Task(task, _) = el {
                    Some(task)
                } else {
                    None
                }
            })
            .collect::<Vec<&Task>>()
    }
    pub fn get_tasks_names(&self) -> Vec<String> {
        self.get_tasks()
            .iter()
            .map(|el| el.get_name().to_owned())
            .collect::<Vec<String>>()
    }
}
