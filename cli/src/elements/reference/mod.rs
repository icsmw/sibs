mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{elements::Element, error::LinkedErr, inf::operator::E};

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Element>,
    pub token: usize,
}

impl Reference {
    fn get_linked_task<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
    ) -> Result<&'a Element, LinkedErr<E>> {
        let (master, task_name) = if self.path.len() == 1 {
            (owner.as_component()?, &self.path[0])
        } else if self.path.len() == 2 {
            let master = if self.path[0] == SELF {
                owner
            } else {
                components
                    .iter()
                    .find(|c| {
                        if let Ok(c) = c.as_component() {
                            c.name.to_string() == self.path[0]
                        } else {
                            false
                        }
                    })
                    .ok_or(E::NotFoundComponent(self.path[0].to_owned()).by(self))?
            };
            (master.as_component()?, &self.path[1])
        } else {
            return Err(E::InvalidPartsInReference.by(self));
        };

        let (task, _) = master.get_task(task_name).ok_or(E::TaskNotExists(
            task_name.to_owned(),
            master.get_name(),
            master.get_tasks_names(),
        ))?;
        Ok(task)
    }
}
