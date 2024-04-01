use crate::{
    executors,
    inf::{context::E, tracker, Context, Scenario, Term, Tracker},
    reader::sources::Sources,
};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct Create;
impl Create {
    fn register_functions(mut cx: Context) -> Result<Context, E> {
        executors::register(&mut cx)?;
        Ok(cx)
    }
    pub fn bound(&self, scenario: &PathBuf) -> Result<Context, E> {
        let tracker = Tracker::new(tracker::Configuration::default());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(
                scenario
                    .parent()
                    .ok_or(E::NoParentFolderFor(scenario.to_owned()))?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(scenario)?,
            tracker,
            term: Term::new(),
            sources: Sources::new(),
            variables: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn unbound(&self) -> Result<Context, E> {
        let tracker = Tracker::new(tracker::Configuration::logs());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker,
            sources: Sources::new(),
            term: Term::new(),
            variables: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn with_tracker(&self, tracker: Tracker) -> Result<Context, E> {
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker,
            sources: Sources::new(),
            term: Term::new(),
            variables: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
}
