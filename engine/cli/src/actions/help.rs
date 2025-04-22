use crate::*;

pub struct HelpAction {
    pub inner: bool,
}

impl ActionMethods for HelpAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(vec![ActionArtifact::HelpRequest])
    }
    fn run(&self, artifacts: Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        if self.inner {
            let lines: Vec<String> = Parameters::as_vec()
                .iter()
                .map(|param| format!("[b]{}[/b][>>]: {}", param.key().join(", "), param.desc()))
                .collect();
            term::print(lines.join("\n"));
            return Ok(RunArtifact::Void);
        }
        let component = artifacts.iter().find_map(|art| {
            if let ActionArtifact::Component(component) = art {
                Some(component.to_string())
            } else {
                None
            }
        });
        let task = artifacts.iter().find_map(|art| {
            if let ActionArtifact::Task(task, ..) = art {
                Some(task.to_string())
            } else {
                None
            }
        });
        let scenario = if let Some(ActionArtifact::Scenario(scenario)) = artifacts
            .into_iter()
            .find(|art| matches!(art, ActionArtifact::Scenario(..)))
        {
            scenario
        } else {
            Scenario::new()?
        };
        let script = Script::new(scenario, component, task, None)?;
        script.print()?;
        Ok(RunArtifact::Void)
    }
}
