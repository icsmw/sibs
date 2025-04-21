mod component;
mod help;
mod scenario;
mod task;
mod version;

use std::{future::Future, pin::Pin};

use crate::*;

pub use component::*;
pub use help::*;
pub use scenario::*;
pub use task::*;
pub use version::*;

pub type ActionPinnedResult<'a> = Pin<Box<dyn Future<Output = ActionResult> + 'a + Send>>;
pub type ActionResult = Result<(), E>;

#[derive(Clone)]
pub enum ActionArtifact {
    Scenario(Scenario),
    /// * `String` - name of task
    /// * `Vec<String>` - task's arguments
    Task(String, Vec<String>),
    /// `String` - name of component
    Component(String),
    HelpRequest,
}

pub trait ActionMethods {
    fn validate(&self, actions: &[Action]) -> Result<(), E> {
        Ok(())
    }
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(Vec::new())
    }
    fn no_context_run(&self, _artifacts: &[ActionArtifact]) -> Result<(), E> {
        Ok(())
    }
    fn context_run(&self, _artifacts: Vec<ActionArtifact>) -> ActionPinnedResult {
        Box::pin(async move { Ok(()) })
    }
}

pub enum Action {
    Help(HelpAction),
    Scenario(ScenarioAction),
    Component(ComponentAction),
    Task(TaskAction),
    Version(VersionAction),
}

impl ActionMethods for Action {
    fn validate(&self, actions: &[Action]) -> Result<(), E> {
        match self {
            Self::Help(act) => act.validate(actions),
            Self::Scenario(act) => act.validate(actions),
            Self::Component(act) => act.validate(actions),
            Self::Task(act) => act.validate(actions),
            Self::Version(act) => act.validate(actions),
        }
    }
    fn artifact(&self, actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        match self {
            Self::Help(act) => act.artifact(actions),
            Self::Scenario(act) => act.artifact(actions),
            Self::Component(act) => act.artifact(actions),
            Self::Task(act) => act.artifact(actions),
            Self::Version(act) => act.artifact(actions),
        }
    }
    fn no_context_run(&self, artifacts: &[ActionArtifact]) -> Result<(), E> {
        match self {
            Self::Help(act) => act.no_context_run(artifacts),
            Self::Scenario(act) => act.no_context_run(artifacts),
            Self::Component(act) => act.no_context_run(artifacts),
            Self::Task(act) => act.no_context_run(artifacts),
            Self::Version(act) => act.no_context_run(artifacts),
        }
    }
    #[boxed]
    fn context_run(&self, artifacts: Vec<ActionArtifact>) -> ActionPinnedResult {
        match self {
            Self::Help(act) => act.context_run(artifacts).await,
            Self::Scenario(act) => act.context_run(artifacts).await,
            Self::Component(act) => act.context_run(artifacts).await,
            Self::Task(act) => act.context_run(artifacts).await,
            Self::Version(act) => act.context_run(artifacts).await,
        }
    }
}
