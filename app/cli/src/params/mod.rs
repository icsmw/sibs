mod help;
mod lsp;
mod scenario;
mod sessions;
mod version;

use crate::*;

pub(crate) use help::*;
pub(crate) use lsp::*;
pub(crate) use scenario::*;
pub(crate) use sessions::*;
pub(crate) use version::*;

pub trait Parameter {
    fn keys() -> Vec<String>;
    fn desc() -> String;
    fn action(args: &mut Vec<String>) -> Option<Result<Action, E>>;
}

#[enum_ids::enum_ids(iterator)]
pub enum Parameters {
    Help,
    Scenario,
    Version,
    Sessions,
    Lsp,
}

impl Parameters {
    pub fn key(&self) -> Vec<String> {
        match self {
            Self::Help => HelpParameter::keys(),
            Self::Scenario => ScenarioParameter::keys(),
            Self::Version => VersionParameter::keys(),
            Self::Sessions => SessionsParameter::keys(),
            Self::Lsp => LspParameter::keys(),
        }
    }
    pub fn desc(&self) -> String {
        match self {
            Self::Help => HelpParameter::desc(),
            Self::Scenario => ScenarioParameter::desc(),
            Self::Version => VersionParameter::desc(),
            Self::Sessions => SessionsParameter::desc(),
            Self::Lsp => LspParameter::desc(),
        }
    }
    pub fn actions() -> Result<Vec<Action>, E> {
        let mut args: Vec<String> = std::env::args().map(|arg| arg.to_string()).collect();
        if !args.is_empty() {
            let _ = args.remove(0);
        }
        let mut actions = Vec::new();
        for param in Parameters::as_vec() {
            if let Some(action) = match param {
                Parameters::Help => HelpParameter::action(&mut args),
                Parameters::Scenario => ScenarioParameter::action(&mut args),
                Parameters::Version => VersionParameter::action(&mut args),
                Parameters::Sessions => SessionsParameter::action(&mut args),
                Parameters::Lsp => LspParameter::action(&mut args),
            } {
                actions.push(action?);
            }
        }
        if !args.is_empty() {
            actions.push(Action::Component(ComponentAction::new(&mut args)?));
        }
        if !args.is_empty() {
            actions.push(Action::Task(TaskAction::new(args)?));
        }
        Ok(actions)
    }
}
