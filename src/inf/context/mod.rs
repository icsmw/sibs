mod error;
use crate::{
    executors::{self, ExecutorFn},
    inf::{
        any::AnyValue,
        scenario::Scenario,
        term::Term,
        tracker::{Logger, Tracker},
    },
};
pub use error::E;
use std::{
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
};

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    vars: HashMap<String, AnyValue>,
    executors: HashMap<String, ExecutorFn>,
    logger: Logger,
}

impl Context {
    fn register_functions(mut cx: Self) -> Result<Self, E> {
        executors::register(&mut cx)?;
        Ok(cx)
    }

    pub fn new(term: Term, tracker: Tracker, scenario: Scenario) -> Result<Self, E> {
        let logger = tracker.get_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: None,
            scenario,
            tracker,
            term,
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn from_filename(filename: &PathBuf) -> Result<Self, E> {
        let tracker = Tracker::new();
        let logger = tracker.get_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(
                filename
                    .parent()
                    .ok_or(E::NoParentFolderFor(filename.to_string_lossy().to_string()))?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(filename)?,
            tracker,
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }

    pub fn unbound() -> Result<Self, E> {
        let tracker = Tracker::new();
        let logger = tracker.get_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker,
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }

    pub fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), E> {
        if let Some(cwd) = cwd.as_ref() {
            self.cwd = Some(self.scenario.to_abs_path(cwd)?);
        } else {
            self.cwd = None;
        }
        Ok(())
    }

    pub fn add_fn(&mut self, name: String, func: ExecutorFn) -> Result<(), E> {
        if let Entry::Vacant(e) = self.executors.entry(name.clone()) {
            e.insert(func);
            Ok(())
        } else {
            Err(E::FunctionAlreadyExists(name))
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<&ExecutorFn> {
        self.executors.get(name)
    }

    pub fn get_var(&self, name: &str) -> Option<&AnyValue> {
        self.vars.get(name)
    }

    pub async fn set_var(&mut self, name: String, value: AnyValue) {
        self.logger
            .log(format!("Assignation: ${name} = {value}"))
            .await;
        self.vars.insert(name, value);
    }
}
