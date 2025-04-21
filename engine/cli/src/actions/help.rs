use crate::*;

pub struct HelpAction {}

impl ActionMethods for HelpAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(vec![ActionArtifact::HelpRequest])
    }
}
