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
    fn run(&self, artifacts: Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        if artifacts
            .iter()
            .any(|art| matches!(art, ActionArtifact::HelpRequest))
        {
            return Ok(RunArtifact::Void);
        }
        let component = if let Some(ActionArtifact::Component(component)) = artifacts
            .iter()
            .find(|art| matches!(art, ActionArtifact::Component(..)))
            .cloned()
        {
            component
        } else {
            return Err(E::NoComponentParameter);
        };
        let scenario = if let Some(ActionArtifact::Scenario(scenario)) = artifacts
            .into_iter()
            .find(|art| matches!(art, ActionArtifact::Scenario(..)))
        {
            scenario
        } else {
            Scenario::new()?
        };
        Ok(RunArtifact::Script(Script::new(
            scenario,
            Some(component),
            Some(self.name.clone()),
            Some(self.args.clone()),
        )?))
    }
}
