mod error;
use crate::{
    executors::{self, ExecutorFn},
    inf::{
        any::AnyValue,
        scenario::Scenario,
        term::Term,
        tracker::{self, Logger, Logs, Tracker},
    },
    reader::map::Map,
};
pub use error::E;
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Context {
    pub cwd: Option<PathBuf>,
    pub term: Term,
    pub tracker: Tracker,
    pub scenario: Scenario,
    pub map: Map,
    vars: HashMap<String, AnyValue>,
    executors: HashMap<String, ExecutorFn>,
    logger: Logger,
}

impl Context {
    fn register_functions(mut cx: Self) -> Result<Self, E> {
        executors::register(&mut cx)?;
        Ok(cx)
    }

    pub fn with_tracker(tracker: Tracker) -> Result<Self, E> {
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: None,
            scenario: Scenario::dummy(),
            tracker,
            term: Term::new(),
            map: Map::new(String::new()),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }
    pub fn from_filename(filename: &Path) -> Result<Self, E> {
        let tracker = Tracker::new(tracker::Configuration::default());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(
                filename
                    .parent()
                    .ok_or(E::NoParentFolderFor(filename.to_string_lossy().to_string()))?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(filename)?,
            tracker,
            map: Map::new(String::new()),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }

    pub fn unbound() -> Result<Self, E> {
        let tracker = Tracker::new(tracker::Configuration::default());
        let logger = tracker.create_logger(String::from("Context"));
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker,
            map: Map::new(String::new()),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
            logger,
        })
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = map;
    }

    pub fn set_scenario(&mut self, scenario: Scenario) {
        self.scenario = scenario;
    }

    pub async fn set_cwd(&mut self, cwd: Option<PathBuf>) -> Result<(), E> {
        if let Some(cwd) = cwd.as_ref() {
            let cwd = self.scenario.to_abs_path(cwd)?;
            self.logger
                .log(format!("cwd: {}", cwd.to_string_lossy()))
                .await;
            self.cwd = Some(cwd);
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

    pub async fn get_var(&self, name: &str) -> Option<&AnyValue> {
        self.logger
            .log(format!("Reading variable: ${name};",))
            .await;
        if !self.vars.contains_key(name) {
            self.logger
                .err(format!("Variable: ${name} doesn't exist;"))
                .await;
        }
        self.vars.get(name)
    }

    pub async fn set_var(&mut self, name: String, value: AnyValue) {
        self.logger
            .log(format!("Assignation: ${name} = {value}"))
            .await;
        self.vars.insert(name, value);
    }
}
