mod component;
mod help;
mod scenario;
mod task;
mod version;

use crate::*;

pub use component::*;
pub use help::*;
pub use scenario::*;
pub use task::*;
pub use version::*;

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

#[allow(clippy::large_enum_variant)]
pub enum RunArtifact {
    Script(Script),
    Void,
}

pub trait ActionMethods {
    fn validate(&self, _actions: &[Action]) -> Result<(), E> {
        Ok(())
    }
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(Vec::new())
    }
    fn run(&self, _artifacts: Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        Ok(RunArtifact::Void)
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
    fn run(&self, artifacts: Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        match self {
            Self::Help(act) => act.run(artifacts),
            Self::Scenario(act) => act.run(artifacts),
            Self::Component(act) => act.run(artifacts),
            Self::Task(act) => act.run(artifacts),
            Self::Version(act) => act.run(artifacts),
        }
    }
}
