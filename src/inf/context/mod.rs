mod error;
use crate::{
    executors::{self, ExecutorFn},
    inf::{any::AnyValue, scenario::Scenario, term::Term, tracker::Tracker},
};
pub use error::E;
use std::{
    any::Any,
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
}

impl Context {
    fn register_functions(mut cx: Self) -> Result<Self, E> {
        executors::register(&mut cx)?;
        Ok(cx)
    }

    pub fn new(term: Term, tracker: Tracker, scenario: Scenario) -> Result<Self, E> {
        Self::register_functions(Context {
            cwd: None,
            scenario,
            tracker,
            term,
            vars: HashMap::new(),
            executors: HashMap::new(),
        })
    }
    pub fn from_filename(filename: &PathBuf) -> Result<Self, E> {
        Self::register_functions(Context {
            cwd: Some(
                filename
                    .parent()
                    .ok_or(E::NoParentFolderFor(filename.to_string_lossy().to_string()))?
                    .to_path_buf(),
            ),
            scenario: Scenario::from(filename)?,
            tracker: Tracker::new(),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
        })
    }

    pub fn unbound() -> Result<Self, E> {
        Self::register_functions(Context {
            cwd: Some(PathBuf::new()),
            scenario: Scenario::dummy(),
            tracker: Tracker::new(),
            term: Term::new(),
            vars: HashMap::new(),
            executors: HashMap::new(),
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

    pub fn set_var(&mut self, name: String, value: AnyValue) {
        self.vars.insert(name, value);
    }
}
