use crate::*;

#[derive(Default)]
pub struct LspAction {}

impl ActionMethods for LspAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(Vec::new())
    }
    fn run(&self, _artifacts: Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        Ok(RunArtifact::Lsp)
    }
}
