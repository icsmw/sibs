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
    fn no_context_run(&self, _: &[ActionArtifact]) -> Result<(), E> {
        println!("{VERSION}");
        Ok(())
    }
}
