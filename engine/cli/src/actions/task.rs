use crate::*;

pub struct TaskAction {
    pub name: String,
    pub args: Vec<String>,
}

impl TaskAction {
    pub fn new(mut args: Vec<String>) -> Result<Self, E> {
        if args.is_empty() {
            return Err(E::FailToGetTaskName);
        }
        Ok(Self {
            name: args.remove(0),
            args,
        })
    }
}

impl ActionMethods for TaskAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(vec![ActionArtifact::Task(
            self.name.clone(),
            self.args.clone(),
        )])
    }
}
