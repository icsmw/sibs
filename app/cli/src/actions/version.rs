use crate::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct VersionAction {}

impl ActionMethods for VersionAction {
    fn validate(&self, actions: &[Action]) -> Result<(), E> {
        if actions.len() != 1 {
            Err(E::StandaloneParameter(
                Parameters::Version.key().join(", ").to_string(),
            ))
        } else {
            Ok(())
        }
    }
    fn run(&self, _artifacts: &mut Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        println!("{VERSION}");
        Ok(RunArtifact::Void)
    }
}
