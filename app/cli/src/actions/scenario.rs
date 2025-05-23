use crate::*;

pub struct ScenarioAction {
    pub filepath: String,
}

impl ActionMethods for ScenarioAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(vec![ActionArtifact::Scenario(Scenario::from(
            &self.filepath,
        )?)])
    }
}
