mod component;
mod help;
mod lsp;
mod scenario;
mod sessions;
mod task;
mod version;

use core::fmt;

use crate::*;

pub(crate) use component::*;
pub(crate) use help::*;
pub(crate) use lsp::*;
pub(crate) use scenario::*;
pub(crate) use sessions::*;
pub(crate) use task::*;
pub(crate) use version::*;

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
    Lsp,
    Void,
}

pub trait ActionMethods {
    fn validate(&self, _actions: &[Action]) -> Result<(), E> {
        Ok(())
    }
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(Vec::new())
    }
    fn run(&self, _artifacts: &mut Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        Ok(RunArtifact::Void)
    }
}

pub enum Action {
    Help(HelpAction),
    Scenario(ScenarioAction),
    Component(ComponentAction),
    Task(TaskAction),
    Version(VersionAction),
    Sessions(SessionsAction),
    Lsp(LspAction),
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Help(_) => write!(f, "HelpAction"),
            Self::Scenario(_) => write!(f, "ScenarioAction"),
            Self::Component(_) => write!(f, "ComponentAction"),
            Self::Task(_) => write!(f, "TaskAction"),
            Self::Version(_) => write!(f, "VersionAction"),
            Self::Sessions(_) => write!(f, "SessionsAction"),
            Self::Lsp(_) => write!(f, "LspAction"),
        }
    }
}

impl ActionMethods for Action {
    fn validate(&self, actions: &[Action]) -> Result<(), E> {
        match self {
            Self::Help(act) => act.validate(actions),
            Self::Scenario(act) => act.validate(actions),
            Self::Component(act) => act.validate(actions),
            Self::Task(act) => act.validate(actions),
            Self::Version(act) => act.validate(actions),
            Self::Sessions(act) => act.validate(actions),
            Self::Lsp(act) => act.validate(actions),
        }
    }
    fn artifact(&self, actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        match self {
            Self::Help(act) => act.artifact(actions),
            Self::Scenario(act) => act.artifact(actions),
            Self::Component(act) => act.artifact(actions),
            Self::Task(act) => act.artifact(actions),
            Self::Version(act) => act.artifact(actions),
            Self::Sessions(act) => act.artifact(actions),
            Self::Lsp(act) => act.artifact(actions),
        }
    }
    fn run(&self, artifacts: &mut Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        match self {
            Self::Help(act) => act.run(artifacts),
            Self::Scenario(act) => act.run(artifacts),
            Self::Component(act) => act.run(artifacts),
            Self::Task(act) => act.run(artifacts),
            Self::Version(act) => act.run(artifacts),
            Self::Sessions(act) => act.run(artifacts),
            Self::Lsp(act) => act.run(artifacts),
        }
    }
}
